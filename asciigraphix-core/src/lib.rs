use std::slice::Iter;

use shapes::{Edge, Point};

pub mod shapes;

#[derive(Clone)]
struct Cell {
    pub x: usize,
    pub y: usize,
    pub char: char,
    pub color: TextColor
}

impl Cell {
    pub fn new(x: usize, y: usize, char: char) -> Self {
        Self { x, y, char, color: TextColor::Red }
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
enum TextColor {
    Red,
    Cyan,
    BrightCyan,
    BrightGreen,
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

    fn clear_screen() {
        // clear screen
        print!("\x1B[2J\x1B[1;1H");
        // hide cursor
        print!("\x1B[?25l");
    }

    fn project_point(&mut self, point: &Point) {
        let cam_to_point = point.clone() - self.cam_pos.clone();

        let z = cam_to_point.dot(&self.cam_unit_vectors.0);
        if z <= 0.0 {
            return;
        }

        let x = cam_to_point.dot(&self.cam_unit_vectors.1);
        let y = cam_to_point.dot(&self.cam_unit_vectors.2);

        let x_pixel = self.cam_focal * x / z;
        let y_pixel = self.cam_focal * y / z;

        let x_pixel = x_pixel + (self.width as f64 / 2.0);
        let y_pixel = y_pixel + (self.height as f64 / 2.0);

        if (x_pixel < 0.0) || (y_pixel < 0.0) {
            return;
        }

        let x_pixel = x_pixel.round() as usize;
        let y_pixel = y_pixel.round() as usize;

        if (x_pixel >= self.width) || (y_pixel >= self.height) {
            return;
        }

        let cur_depth = self.z_buffer.xy(x_pixel, y_pixel);
        match cur_depth {
            Some(k) if z > k => self.z_buffer.set_xy(x_pixel, y_pixel, Some(z)),
            None => self.z_buffer.set_xy(x_pixel, y_pixel, Some(z)),
            _ => {
                return;
            }
        }

        let cell = self.frame_buffer.xy_mut(x_pixel, y_pixel);
        let color = TextColor::Cyan;
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

    // fn project_faces(&mut self, vertices: &Vec<Point>, faces: &Vec<Face>) {
    //     for face in faces {
    //         let vertex_1 = vertices.get(face.0);
    //         let vertex_2 = vertices.get(face.1);
    //         let vertex_3 = vertices.get(face.2);
    //     }
    // }

    fn project(&mut self, shape: &shapes::Shape) {
        self.project_vertices(&shape.vertices);
        self.project_edges(&shape.vertices, &shape.edges);
        // self.project_faces(&shape.vertices, &shape.faces);
    }

    fn colored(text: &str, color: TextColor) {
        match color {
            TextColor::Red => print!("\x1b[31m{}\x1b[0m", text),
            TextColor::Cyan => print!("\x1b[36m{}\x1b[0m", text),
            TextColor::BrightCyan => print!("\x1b[96m{}\x1b[0m", text),
            TextColor::BrightGreen => print!("\x1b[92m{}\x1b[0m", text),
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
        for Cell { x, y, char, color } in self.frame_buffer.iter() {
            let z = self.z_buffer.xy(*x, *y);
            match (*x, char) {
                (0, _) => Self::colored("|", *color),
                (i, _) if i == self.width - 1 => {
                    Self::colored("|\r\n", *color);
                }
                (_, char) => {Self::colored(&char.to_string(), *color);}
            }
        }
    }
}
