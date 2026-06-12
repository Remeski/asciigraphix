use std::slice::Iter;

use rand::RngExt;
use shapes::Edge;

use crate::{math::Point, shapes::Face};

pub mod math;
pub mod shapes;

#[derive(Clone)]
struct Cell {
    pub x: usize,
    pub y: usize,
    pub char: char,
    pub color: TextColor,
}

impl Cell {
    pub fn new(x: usize, y: usize, char: char) -> Self {
        Self {
            x,
            y,
            char,
            color: TextColor::Red,
        }
    }
}

#[derive(Default, Clone)]
struct FrameBuffer {
    width: usize,
    height: usize,
    buffer: Vec<Cell>,
}

#[derive(Default, Clone)]
struct ZBuffer {
    width: usize,
    height: usize,
    buffer: Vec<Option<f64>>,
}

impl<'a> FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: Self::init_buffer(width, height),
        }
    }

    fn init_buffer(width: usize, height: usize) -> Vec<Cell> {
        let mut buffer = Vec::new();
        for y in 0..height {
            for x in 0..width {
                buffer.push(Cell::new(x, y, ' '));
            }
        }
        buffer
    }

    pub fn clear(&mut self) {
        let width = self.width;
        let height = self.height;
        for y in 0..height {
            for x in 0..width {
                let idx = self.idx(x, y);
                let c = self.buffer.get_mut(idx).expect("should be inside");
                c.char = ' '
            }
        }
    }

    pub const fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn xy(&'a self, x: usize, y: usize) -> &'a Cell {
        let idx = self.idx(x, y);
        &self.buffer[idx]
    }

    pub fn xy_mut(&'a mut self, x: usize, y: usize) -> &'a mut Cell {
        let idx = self.idx(x, y);
        &mut self.buffer[idx]
    }

    pub fn set_xy(&mut self, x: usize, y: usize, cell: Cell) {
        let idx = self.idx(x, y);
        self.buffer[idx] = cell;
    }

    pub fn iter(&self) -> Iter<'_, Cell> {
        self.buffer.iter()
    }
}

impl ZBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![None; width * height],
        }
    }

    pub fn clear(&mut self) {
        let width = self.width;
        let height = self.height;
        for y in 0..height {
            for x in 0..width {
                let idx = self.idx(x, y);
                let _ = self.buffer.get_mut(idx).expect("should be inside").take();
            }
        }
    }

    pub const fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn xy(&self, x: usize, y: usize) -> Option<f64> {
        let idx = y * self.width + x;
        self.buffer[idx]
    }

    pub fn set_xy(&mut self, x: usize, y: usize, depth: Option<f64>) {
        let idx = self.idx(x, y);
        self.buffer[idx] = depth;
    }
}

// 8-bit color
pub struct RGB(u8, u8, u8);

impl RGB {
    pub fn to_u32(&self) -> u32 {
        (self.0 as u32) << 16 | (self.1 as u32) << 8 | self.2 as u32
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum TextColor {
    Red,
    Cyan,
    BrightCyan,
    BrightGreen,
    Rgb(u8, u8, u8),
}

enum ViewportPoint {
    Inside(Point),
    Outside(Point),
    Behind,
}

pub struct Display {
    width: usize,
    height: usize,
    z_buffer: ZBuffer,
    frame_buffer: FrameBuffer,
    pub cam_pos: Point,
    cam_unit_vectors: (Point, Point, Point),
    cam_focal: f64,
}

impl Display {
    pub fn new(
        width: usize,
        height: usize,
        cam_pos: Point,
        cam_direction: Point,
        cam_vup: Point,
        cam_fov: f64,
    ) -> Display {
        Display {
            width,
            height,
            z_buffer: ZBuffer::new(width, height),
            frame_buffer: FrameBuffer::new(width, height),
            cam_pos,
            cam_unit_vectors: Self::cam_basis(cam_direction, cam_vup),
            cam_focal: height as f64 / 2.0 * (cam_fov / 2.0).tan(),
        }
    }

    fn cam_basis(cam_direction: Point, cam_vup: Point) -> (Point, Point, Point) {
        let a = cam_direction.cross(&cam_vup).unit();
        let b = cam_direction.cross(&a).unit();
        (cam_direction.unit(), a, b)
    }

    fn world_to_viewport(&self, point: &Point) -> ViewportPoint {
        let cam_to_point = point.clone() - self.cam_pos.clone();

        let z = cam_to_point.dot(&self.cam_unit_vectors.0);
        if z <= 0.0 {
            return ViewportPoint::Behind;
        }

        let x = cam_to_point.dot(&self.cam_unit_vectors.1);
        let y = cam_to_point.dot(&self.cam_unit_vectors.2);

        let x = self.cam_focal * x / z;
        let y = self.cam_focal * y / z;

        let x = x + (self.width as f64 / 2.0);
        let y = y + (self.height as f64 / 2.0);

        if (x < 0.0) || (y < 0.0) || (x > self.width as f64 - 1.0) || (y > self.height as f64 - 1.0)
        {
            return ViewportPoint::Outside(Point(x, y, z));
        }

        ViewportPoint::Inside(Point(x, y, z))
    }

    fn set_screen_point(&mut self, x: f64, y: f64, z: f64, color: Option<TextColor>) {
        let x = x.floor() as isize;
        let y = y.floor() as isize;

        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        let cur_depth = self.z_buffer.xy(x, y);
        match cur_depth {
            Some(k) if z < k => self.z_buffer.set_xy(x as usize, y as usize, Some(z)),
            None => self.z_buffer.set_xy(x, y, Some(z)),
            _ => {
                return;
            }
        }

        let cell = self.frame_buffer.xy_mut(x, y);
        let color = color.unwrap_or(TextColor::Red);
        // let color = TextColor::Rgb(((1.0 - z.clamp(0.0, 20.0) / 20.0) * 249.0).round() as u8, 0, 0);
        // if z < 10.0 {
        //     cell.char = '#';
        //     cell.color = color;
        // } else if z < 30.0 {
        //     cell.char = '*';
        //     cell.color = color;
        // } else if z < 50.0 {
        //     cell.char = '-';
        //     cell.color = color;
        // } else {
        //     cell.char = '.';
        //     cell.color = color;
        // }
        cell.char = rand::rng().sample(rand::distr::Alphabetic) as char;
        // cell.char = '█';
        cell.color = color;
        // cell.char = (b'0' + (1.0 / z).round().abs() as u8) as char
        // cell.char = (b'0' + (y - 18) as u8) as char;
    }

    fn draw_vertex(&mut self, point: &Point, color: Option<TextColor>) {
        if let ViewportPoint::Inside(Point(x, y, z)) = self.world_to_viewport(point) {
            self.set_screen_point(x, y, z, color);
        }
    }

    fn transform_vertices(&mut self, vertices: &Vec<Point>) {
        for vertex in vertices {
            self.draw_vertex(vertex, None);
        }
    }

    fn transform_edges(&mut self, vertices: &Vec<Point>, edges: &Vec<Edge>) {
        for edge in edges {
            let start = vertices[edge.0].clone();
            let end = vertices[edge.1].clone();

            let delta = end.clone() - start.clone();

            const VERTEX_DENSITY: usize = 100;
            for c in 0..VERTEX_DENSITY {
                let point = start.clone() + delta.clone() * (c as f64 / VERTEX_DENSITY as f64);
                self.draw_vertex(&point, edge.2.clone().map(|t| t.into()));
            }
        }
    }

    fn draw_scanline(
        &mut self,
        mut x_start: f64,
        mut x_end: f64,
        y: f64,
        Point(x1, y1, z1): Point,
        Point(x2, y2, z2): Point,
        Point(x_new, y_new, z_new): Point,
        color: Option<TextColor>,
    ) {
        if x_start > x_end {
            std::mem::swap(&mut x_start, &mut x_end);
        }

        for x in x_start.floor() as isize..(x_end.ceil() + 1.0) as isize {
            let bary = Point(x as f64, y as f64, 0.0).to_barycentric(
                Point(x1, y1, 0.0),
                Point(x2, y2, 0.0),
                Point(x_new, y_new, 0.0),
            );

            if bary.0 * bary.1 >= 0.0 && bary.0 * bary.2 >= 0.0 && bary.1 * bary.2 >= 0.0 {
                let z_new = bary.0 * 1.0 / z1 + bary.1 * 1.0 / z2 + bary.2 * 1.0 / z_new;
                self.set_screen_point(x as f64, y as f64, 1.0 / z_new, color);
            }
        }
    }

    /// y_new and y2 must be the same (i.e. the line between (x2,y2) and (x_new, y_new) is
    /// horizontal).
    /// also y1 < y2 (i.e. on the screen (x1, y1) will be on top).
    fn draw_toptriangle(&mut self, v1: Point, v2: Point, v_new: Point, color: Option<TextColor>) {
        let Point(x1, y1, _) = v1;
        let Point(x2, y2, _) = v2;
        let Point(x_new, y_new, _) = v_new;

        let invslope1 = (x2 - x1) / (y2 - y1);
        let invslope2 = (x_new - x1) / (y_new - y1);

        let mut x_start = x1;
        let mut x_end = x1;

        for y in (y1.floor() as isize)..(y_new.ceil() as isize) {
            self.draw_scanline(x_start, x_end, y as f64, v1, v2, v_new, color);
            x_start += invslope1;
            x_end += invslope2;
        }
    }

    /// y_new and y2 must be the same (i.e. the line between (x2,y2) and (x_new, y_new) is
    /// horizontal).
    /// also y1 > y2 (i.e. on the screen (x1, y1) will be on bottom).
    fn project_bottomtriangle(
        &mut self,
        v1: Point,
        v2: Point,
        v_new: Point,
        color: Option<TextColor>,
    ) {
        let Point(x1, y1, _) = v1;
        let Point(x2, y2, _) = v2;
        let Point(x_new, y_new, _) = v_new;

        let invslope1 = (x1 - x2) / (y1 - y2);
        let invslope2 = (x1 - x_new) / (y1 - y_new);

        let mut x_start = x2;
        let mut x_end = x_new;

        for y in (y_new.floor() as isize)..(y1.ceil() as isize) {
            // dbg!(x_start, x_end);
            self.draw_scanline(x_start, x_end, y as f64, v1, v2, v_new, color);
            x_start += invslope1;
            x_end += invslope2;
        }
    }

    /// Expects sorted along y-axis (first highest on the screen which means the actual value of y1
    /// is smallest).
    fn draw_triangle(
        &mut self,
        mut v1: Point,
        mut v2: Point,
        mut v3: Point,
        color: Option<TextColor>,
    ) {
        if v1.1 > v2.1 {
            std::mem::swap(&mut v1, &mut v2);
        }
        if v1.1 > v3.1 {
            std::mem::swap(&mut v1, &mut v3);
        }
        if v2.1 > v3.1 {
            std::mem::swap(&mut v2, &mut v3);
        }

        let y1y2_close = (v1.1 - v2.1).abs() < 1e-2;
        let y2y3_close = (v2.1 - v3.1).abs() < 1e-2;
        let y1y3_close = (v1.1 - v3.1).abs() < 1e-2;

        if y1y2_close && y2y3_close && y1y3_close {
            return;
        }

        if y2y3_close {
            self.draw_toptriangle(v1, v2, v3, color);
            return;
        }

        if y1y2_close {
            self.project_bottomtriangle(v3, v1, v2, color);
            return;
        }

        let Point(x1, y1, z1) = v1;
        let Point(x2, y2, z2) = v2;
        let Point(x3, y3, z3) = v3;

        let x_new = (x3 - x1) / (y3 - y1) * (y2 - y1) + x1;
        let y_new = y2;
        let bary = Point(x_new, y_new, 0.0).to_barycentric(
            Point(x1, y1, 0.0),
            Point(x2, y2, 0.0),
            Point(x3, y3, 0.0),
        );
        let z_new = 1.0 / (bary.0 * 1.0 / z1 + bary.1 * 1.0 / z2 + bary.2 * 1.0 / z3);

        self.draw_toptriangle(v1, v2, Point(x_new, y_new, z_new), color);
        self.project_bottomtriangle(v3, v2, Point(x_new, y_new, z_new), color);
    }

    fn transform_faces(&mut self, vertices: &Vec<Point>, faces: &Vec<Face>) {
        for face in faces {
            let v1 = vertices.get(face.0).unwrap();
            let v2 = vertices.get(face.1).unwrap();
            let v3 = vertices.get(face.2).unwrap();

            let viewport1 = self.world_to_viewport(v1);
            let viewport2 = self.world_to_viewport(v2);
            let viewport3 = self.world_to_viewport(v3);
            match (viewport1, viewport2, viewport3) {
                (
                    ViewportPoint::Inside(v1),
                    ViewportPoint::Inside(v2),
                    ViewportPoint::Inside(v3),
                ) => {
                    self.draw_triangle(v1, v2, v3, face.3.clone().map(|t| t.into()));
                }
                _ => {}
            }
        }
    }

    fn world_to_screen(&mut self, shape: &shapes::Shape) {
        self.transform_faces(&shape.vertices, &shape.faces);
        // self.transform_edges(&shape.vertices, &shape.edges);
        // self.transform_vertices(&shape.vertices);
    }

    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H"); // clear screen
        print!("\x1B[?25l"); // hide cursor
    }

    fn set_terminal_char(x: usize, y: usize, char: &str) {
        // move cursor
        print!("\x1B[{};{}H", y, x);
        print!("\x1B[{};{}f", y, x);
        // delete character
        print!("");
        print!("{}", char);
    }

    fn colored(text: &str, color: TextColor) {
        print!("{}", Self::colored_string(text, color))
    }

    fn colored_string(text: &str, color: TextColor) -> String {
        match color {
            TextColor::Red => format!("\x1b[31m{}\x1b[0m", text),
            TextColor::Cyan => format!("\x1b[36m{}\x1b[0m", text),
            TextColor::BrightCyan => format!("\x1b[96m{}\x1b[0m", text),
            TextColor::BrightGreen => format!("\x1b[92m{}\x1b[0m", text),
            TextColor::Rgb(r, g, b) => format!("\x1b[38;2;{r};{g};{b}m{}\x1b[0m", text),
        }
    }

    // Vec<(depth, color)>
    // for now color is set to some default
    pub fn render(&mut self, shape: &shapes::Shape) -> Vec<(f32, u32)> {
        const FG: RGB = RGB(254, 0, 0);
        const BG: RGB = RGB(10, 10, 10);
        self.world_to_screen(&shape);
        let mut result = Vec::new();

        for Cell { x, y, .. } in self.frame_buffer.iter() {
            let z = self.z_buffer.xy(*x, *y);
            match z {
                Some(p) => {
                    result.push(((p as f32).clone(), FG.to_u32()));
                }
                None => {
                    result.push((0.0 as f32, BG.to_u32()));
                }
            }
        }

        result
    }

    pub fn render_print(&mut self, shape: &shapes::Shape) {
        self.frame_buffer.clear();
        self.z_buffer.clear();
        self.world_to_screen(&shape);
        Self::clear_screen();
        for Cell {
            x,
            y: _,
            char,
            color,
        } in self.frame_buffer.iter()
        {
            match (*x, char) {
                // (0, _) => Self::colored("|", *color),
                (i, _) if i == self.width - 1 => {
                    Self::colored("\r\n", *color);
                }
                (_, char) => {
                    Self::colored(&char.to_string(), *color);
                }
            }
        }
    }

    pub fn render_terminal(&mut self, shape: &shapes::Shape) {
        self.frame_buffer.clear();
        self.z_buffer.clear();
        self.world_to_screen(&shape);
        for Cell { x, y, char, color } in self.frame_buffer.iter() {
            Self::set_terminal_char(*x, *y, &Self::colored_string(&char.to_string(), *color));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RGB;

    #[test]
    fn rgb() {
        let color = RGB(255, 5, 15);
        assert_eq!(color.to_u32(), 0x00FF050F);
    }
}
