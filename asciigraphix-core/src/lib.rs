use shapes::{Edge, Point};

pub mod shapes;

pub struct Display {
    x_size: usize,
    y_size: usize,
    pixels: Vec<Vec<Option<f64>>>,
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
        x_size: usize,
        y_size: usize,
        cam_pos: Point,
        cam_direction: Point,
        cam_focal: f64,
    ) -> Display {
        let pixels: Vec<Vec<Option<f64>>> = vec![vec![None; x_size]; y_size];
        Display {
            x_size,
            y_size,
            pixels,
            cam_pos,
            cam_unit_vectors: Self::orthogonal_basis(cam_direction),
            cam_focal,
        }
    }

    fn orthogonal_basis(cam_direction: Point) -> (Point, Point, Point) {
        // Pick a vector from xy-plane i.e. (x, y, 0) that is orthogonal to cam_direction
        let a = Point(cam_direction.1, -cam_direction.0, 0.0).unit();
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
        // dbg!(self.cam_unit_vectors.clone());
        let depth = cam_to_point.dot(&self.cam_unit_vectors.0);
        let x = cam_to_point.dot(&self.cam_unit_vectors.1);
        let y = cam_to_point.dot(&self.cam_unit_vectors.2);
        if depth <= 0.0 {
            return;
        }
        // let x = self.cam_unit_vectors.clone().1 * dot_x;
        // let y = self.cam_unit_vectors.clone().2 * dot_y;
        // let z = self.cam_unit_vectors.clone().0 * dot_z;

        let x_pixel = self.cam_focal * x / depth;
        let y_pixel = self.cam_focal * y / depth;
        let x_pixel = x_pixel + (self.x_size as f64 / 2.0);
        let y_pixel = y_pixel + (self.y_size as f64 / 2.0);
        if (x_pixel < 0.0) || (y_pixel < 0.0) {
            return;
        }
        let x_pixel = x_pixel.round() as usize;
        let y_pixel = y_pixel.round() as usize;
        if (x_pixel >= self.x_size) || (y_pixel >= self.y_size) {
            return;
        }
        let cur_depth = self.pixels[y_pixel][x_pixel];
        if let Some(k) = cur_depth {
            if depth > k {
                self.pixels[y_pixel][x_pixel] = Some(depth);
            }
        } else {
            self.pixels[y_pixel][x_pixel] = Some(depth);
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
        self.pixels = vec![vec![None; self.x_size]; self.y_size];
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
        const FG: RGB = RGB(254,0,0);
        const BG: RGB = RGB(10,10,10);
        self.project(&shape);
        let mut result = Vec::new();
        for row in &self.pixels {
            for z in row {
                match z {
                    Some(p) => {
                        result.push(((*p as f32).clone(), FG.to_u32()));
                    }
                    None => {
                        result.push((0.0 as f32, BG.to_u32()));
                    }
                }
            }
        }
        result
    }

    pub fn render_print(&mut self, shape: &shapes::Shape) {
        self.project(&shape);
        Self::clear_screen();
        let color = TextColor::Cyan;
        for row in &self.pixels {
            for z in row {
                match z {
                    Some(p) => {
                        if *p < 10.0 {
                            Self::colored("#", color);
                        } else if *p < 30.0 {
                            Self::colored("*", color);
                        } else if *p < 50.0 {
                            Self::colored("-", color);
                        } else {
                            Self::colored(".", color);
                        }
                    }
                    None => {
                        print!(" ");
                    }
                }
            }
            println!()
        }
    }
}
