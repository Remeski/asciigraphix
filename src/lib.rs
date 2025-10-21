use shapes::{Edge, Face, Point};

pub mod shapes;

pub struct Display {
    x_size: usize,
    y_size: usize,
    pixels: Vec<Vec<Option<f64>>>,
    pub cam_pos: Point,
    cam_unit_vectors: (Point, Point, Point),
    cam_focal: f64,
}

#[derive(Copy, Clone)]
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
            cam_unit_vectors: cam_direction.orthogonal_basis(),
            cam_focal,
        }
    }

    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn project_point(&mut self, point: &Point) {
        let cam_to_point = point.clone() - self.cam_pos.clone();
        // dbg!(self.cam_unit_vectors.clone());
        let z = cam_to_point.dot(&self.cam_unit_vectors.0);
        let x = cam_to_point.dot(&self.cam_unit_vectors.1);
        let y = cam_to_point.dot(&self.cam_unit_vectors.2);
        if z <= 0.0 {
            return;
        }
        // let x = self.cam_unit_vectors.clone().1 * dot_x;
        // let y = self.cam_unit_vectors.clone().2 * dot_y;
        // let z = self.cam_unit_vectors.clone().0 * dot_z;

        let x_pixel = self.cam_focal * x / z;
        let y_pixel = self.cam_focal * y / z;
        let x_pixel = x_pixel + (self.x_size as f64 / 2.0);
        let y_pixel = -y_pixel + (self.y_size as f64 / 2.0);
        if (x_pixel < 0.0) || (y_pixel < 0.0) {
            return;
        }
        let x_pixel = x_pixel.round() as usize;
        let y_pixel = y_pixel.round() as usize;
        if (x_pixel >= self.x_size) || (y_pixel >= self.y_size) {
            return;
        }
        let cur_z = self.pixels[y_pixel][x_pixel];
        if let Some(k) = cur_z {
            if z > k {
                self.pixels[y_pixel][x_pixel] = Some(z);
            }
        } else {
            self.pixels[y_pixel][x_pixel] = Some(z);
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

    fn project_faces(&mut self, vertices: &Vec<Point>, faces: &Vec<Face>) {
        for face in faces {
            let vertex_1 = vertices.get(face.0);
            let vertex_2 = vertices.get(face.1);
            let vertex_3 = vertices.get(face.2);
        }
    }

    fn project(&mut self, shape: &shapes::Shape) {
        self.pixels = vec![vec![None; self.x_size]; self.y_size];
        self.project_vertices(&shape.vertices);
        self.project_edges(&shape.vertices, &shape.edges);
        self.project_faces(&shape.vertices, &shape.faces);
    }

    fn colored(text: &str, color: TextColor) {
        match color {
            TextColor::Red => print!("\x1b[31m{}\x1b[0m", text),
            TextColor::Cyan => print!("\x1b[36m{}\x1b[0m", text),
            TextColor::BrightCyan => print!("\x1b[96m{}\x1b[0m", text),
            TextColor::BrightGreen => print!("\x1b[92m{}\x1b[0m", text),
        }
    }

    pub fn render(&mut self, shape: &shapes::Shape) {
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
