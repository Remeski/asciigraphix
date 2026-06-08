use std::slice::Iter;

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
        self.buffer = Self::init_buffer(self.width, self.height);
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

    pub fn iter(&mut self) -> Iter<'_, Cell> {
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
        self.buffer = vec![None; self.width * self.height];
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

pub struct Display {
    width: usize,
    height: usize,
    z_buffer: ZBuffer,
    frame_buffer: FrameBuffer,
    pub cam_pos: Point,
    cam_unit_vectors: (Point, Point, Point),
    cam_focal: f64,
}

// 8-bit color
pub struct RGB(u8, u8, u8);

impl RGB {
    pub fn to_u32(&self) -> u32 {
        (self.0 as u32) << 16 | (self.1 as u32) << 8 | self.2 as u32
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

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum TextColor {
    Red,
    Cyan,
    BrightCyan,
    BrightGreen,
}

enum ViewportPoint {
    Inside(usize, usize, f64),
    Outside(f64, f64, f64),
    Behind,
}

impl Display {
    pub fn new(
        width: usize,
        height: usize,
        cam_pos: Point,
        cam_direction: Point,
        cam_fov: f64,
    ) -> Display {
        Display {
            width,
            height,
            z_buffer: ZBuffer::new(width, height),
            frame_buffer: FrameBuffer::new(width, height),
            cam_pos,
            cam_unit_vectors: Self::orthogonal_basis(cam_direction),
            cam_focal: height as f64 / 2.0 * 1.0 * (cam_fov / 2.0).tan(),
        }
    }

    fn orthogonal_basis(cam_direction: Point) -> (Point, Point, Point) {
        // Pick a vector from xz-plane i.e. (x, 0, z) that is orthogonal to cam_direction
        let a = Point(-cam_direction.2, 0.0, cam_direction.0).unit();
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

        if (x < 0.0) || (y < 0.0) || (x >= self.width as f64) || (y >= self.height as f64) {
            return ViewportPoint::Outside(x, y, z);
        }

        let x_pixel = x.round() as usize;
        let y_pixel = y.round() as usize;

        ViewportPoint::Inside(x_pixel, y_pixel, z)
    }

    fn project_point(&mut self, point: &Point) {
        if let ViewportPoint::Inside(x, y, z) = self.world_to_viewport(point) {
            self.set_viewport_point(x, y, z);
        }
    }

    fn set_viewport_point(&mut self, x: usize, y: usize, z: f64) {
        let cur_depth = self.z_buffer.xy(x, y);
        match cur_depth {
            Some(k) if z > k => self.z_buffer.set_xy(x, y, Some(z)),
            None => self.z_buffer.set_xy(x, y, Some(z)),
            _ => {
                return;
            }
        }

        let cell = self.frame_buffer.xy_mut(x, y);
        let color = TextColor::Red;
        if z < 10.0 {
            cell.char = '#';
            cell.color = color;
        } else if z < 30.0 {
            cell.char = '*';
            cell.color = color;
        } else if z < 50.0 {
            cell.char = '-';
            cell.color = color;
        } else {
            cell.char = '.';
            cell.color = color;
        }
    }

    fn project_vertices(&mut self, vertices: &Vec<Point>) {
        for vertex in vertices {
            self.project_point(vertex);
        }
    }

    fn project_edges(&mut self, vertices: &Vec<Point>, edges: &Vec<Edge>) {
        for edge in edges {
            let start = vertices[edge.0].clone();
            let end = vertices[edge.1].clone();

            let delta = end.clone() - start.clone();
            // let delta_unit = delta.clone() / delta.magnitude();

            const VERTEX_DENSITY: usize = 100;
            for c in 0..VERTEX_DENSITY {
                let point = start.clone() + delta.clone() * (c as f64 / VERTEX_DENSITY as f64);
                self.project_point(&point);
            }
        }
    }

    /// y_new and y2 must be the same (i.e. the line between (x2,y2) and (x_new, y_new) is
    /// horizontal).
    /// also y1 < y2 (i.e. on the screen (x1, y1) will be on top).
    fn project_toptriangle(
        &mut self,
        (x1, y1, z1): (f64, f64, f64),
        (x2, y2, z2): (f64, f64, f64),
        (x_new, y_new, z_new): (f64, f64, f64),
    ) {
        let invslope1 = (x2 - x1) / (y2 - y1);
        let invslope2 = (x_new - x1) / (y_new - y1);

        let mut x_start = x1;
        let mut x_end = x1;

        for y in (y1 as usize)..(y_new as usize) {
            x_start += invslope1;
            x_end += invslope2;

            let dx = x_end - x_start;
            for i in 0..100 {
                let x = x_start + dx * 0.01 * i as f64;
                let x = x.round() as usize;
                self.set_viewport_point(x, y, 1.0);
            }

            // for x in (x_start.round() as usize)..(x_end.round() as usize) {
            //     self.set_viewport_point(x, y, 1.0);
            // }
        }
    }

    /// y_new and y2 must be the same (i.e. the line between (x2,y2) and (x_new, y_new) is
    /// horizontal).
    /// also y1 > y2 (i.e. on the screen (x1, y1) will be on bottom).
    fn project_bottomtriangle(
        &mut self,
        (x1, y1, z1): (f64, f64, f64),
        (x2, y2, z2): (f64, f64, f64),
        (x_new, y_new, z_new): (f64, f64, f64),
    ) {
        let invslope1 = (x1 - x2) / (y1 - y2);
        let invslope2 = (x1 - x_new) / (y1 - y_new);

        let mut x_start = x2;
        let mut x_end = x_new;

        for y in (y_new as usize)..(y1 as usize) {
            x_start += invslope1;
            x_end += invslope2;

            let dx = x_end - x_start;
            for i in 0..100 {
                let x = x_start + dx * 0.01 * i as f64;
                let x = x.round() as usize;
                self.set_viewport_point(x, y, 1.0);
            }

            // let dx = x_start - x_end;
            // for x in (x_start.round() as usize)..(x_end.round() as usize) {
            //     self.set_viewport_point(x, y, 1.0);
            // }
        }
    }

    /// Expects sorted along y-axis (first highest on the screen which means the actual value of y1
    /// is smallest).
    fn project_triangle(
        &mut self,
        (x1, y1, z1): (usize, usize, f64),
        (x2, y2, z2): (usize, usize, f64),
        (x3, y3, z3): (usize, usize, f64),
    ) {
        let (x1, y1, z1) = (x1 as f64, y1 as f64, z1 as f64);
        let (x2, y2, z2) = (x2 as f64, y2 as f64, z2 as f64);
        let (x3, y3, z3) = (x3 as f64, y3 as f64, z3 as f64);

        if y2 == y3 {
            self.project_toptriangle((x1, y1, z1), (x2, y2, z2), (x3, y3, z3));
            return;
        }

        if y1 == y2 {
            self.project_bottomtriangle((x3, y3, z3), (x1, y1, z1), (x2, y2, z2));
            return;
        }

        let x_new = (x3 - x1) / (y3 - y1) * (y2 - y1) + x1;

        let y_new = y2;
        let z_new = z2;

        self.project_toptriangle((x1, y1, z1), (x2, y2, z2), (x_new, y_new, z_new));
        self.project_bottomtriangle((x3, y3, z3), (x2, y2, z2), (x_new, y_new, z_new));
    }

    fn project_faces(&mut self, vertices: &Vec<Point>, faces: &Vec<Face>) {
        for face in faces {
            let vertex1 = self.world_to_viewport(vertices.get(face.0).unwrap());
            let vertex2 = self.world_to_viewport(vertices.get(face.1).unwrap());
            let vertex3 = self.world_to_viewport(vertices.get(face.2).unwrap());

            match (vertex1, vertex2, vertex3) {
                (
                    ViewportPoint::Inside(mut x1, mut y1, z1),
                    ViewportPoint::Inside(mut x2, mut y2, z2),
                    ViewportPoint::Inside(mut x3, mut y3, z3),
                ) => {
                    if y1 > y2 {
                        std::mem::swap(&mut x2, &mut x1);
                        std::mem::swap(&mut y2, &mut y1);
                    }
                    if y1 > y3 {
                        std::mem::swap(&mut x3, &mut x1);
                        std::mem::swap(&mut y3, &mut y1);
                    }
                    if y2 > y3 {
                        std::mem::swap(&mut x2, &mut x3);
                        std::mem::swap(&mut y2, &mut y3);
                    }
                    if y3 == y1 {
                        return;
                    }
                    self.project_triangle((x1, y1, z1), (x2, y2, z2), (x3, y3, z3));
                }
                _ => {}
            }
        }
    }

    fn project(&mut self, shape: &shapes::Shape) {
        self.project_faces(&shape.vertices, &shape.faces);
        self.project_edges(&shape.vertices, &shape.edges);
        self.project_vertices(&shape.vertices);
    }

    pub fn clear_screen() {
        // clear screen
        print!("\x1B[2J\x1B[1;1H");
        // hide cursor
        print!("\x1B[?25l");
    }

    fn set_terminal_char(x: usize, y: usize, char: String) {
        // move cursor
        print!("\x1B[{};{}H", y, x);
        print!("\x1B[{};{}f", y, x);
        // delete character
        print!("");
        print!("{}", char);
    }

    fn colored(text: &str, color: TextColor) {
        match color {
            TextColor::Red => print!("\x1b[31m{}\x1b[0m", text),
            TextColor::Cyan => print!("\x1b[36m{}\x1b[0m", text),
            TextColor::BrightCyan => print!("\x1b[96m{}\x1b[0m", text),
            TextColor::BrightGreen => print!("\x1b[92m{}\x1b[0m", text),
        }
    }

    fn colored_string(text: &str, color: TextColor) -> String {
        match color {
            TextColor::Red => format!("\x1b[31m{}\x1b[0m", text),
            TextColor::Cyan => format!("\x1b[36m{}\x1b[0m", text),
            TextColor::BrightCyan => format!("\x1b[96m{}\x1b[0m", text),
            TextColor::BrightGreen => format!("\x1b[92m{}\x1b[0m", text),
        }
    }

    // Vec<(depth, color)>
    // for now color is set to some default
    pub fn render(&mut self, shape: &shapes::Shape) -> Vec<(f32, u32)> {
        const FG: RGB = RGB(254, 0, 0);
        const BG: RGB = RGB(10, 10, 10);
        self.project(&shape);
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
        self.project(&shape);
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
        self.project(&shape);
        for Cell {
            x,
            y,
            char,
            color,
        } in self.frame_buffer.iter()
        {
            match (*x, char) {
                // (0, _) => Self::colored("|", *color),
                (_, char) => {
                    Self::set_terminal_char(*x, *y, Self::colored_string(&char.to_string(), *color));
                }
            }
        }
    }
}
