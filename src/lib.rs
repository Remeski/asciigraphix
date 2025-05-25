pub mod shapes;

pub struct Display {
    x_size: usize,
    y_size: usize,
    pixels: Vec<Vec<Option<f64>>>,
}

impl Display {
    pub fn new(x_size: usize, y_size: usize) -> Display {
        let pixels: Vec<Vec<Option<f64>>> = vec![vec![None; x_size]; y_size];
        Display {
            x_size,
            y_size,
            pixels,
        }
    }

    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn write_pixel(&mut self, x: f64, y: f64, value: f64) {
        let xpixel = (x + (self.x_size as f64) / 2.0) as isize;
        let ypixel = (y + (self.y_size as f64) / 2.0) as isize;
        if (xpixel < 0) || (ypixel < 0) {
            return;
        }
        let x = xpixel as usize;
        let y = ypixel as usize;
        if (x >= self.x_size) || (y >= self.y_size) {
            return;
        }
        let z = value;
        let cur_z = self.pixels[y][x];
        if let Some(k) = cur_z {
            if z > k {
                self.pixels[y][x] = Some(z);
            }
        } else {
            self.pixels[y][x] = Some(z);
        }
    }

    fn project(&mut self, shape: &shapes::Shape) {
        self.pixels = vec![vec![None; self.x_size]; self.y_size];
        let vertices = &shape.vertices;
        let edges = &shape.edges;

        for vertex in vertices {
            self.write_pixel(vertex.0, vertex.1, vertex.2);
        }

        for edge in edges {
            let start = vertices[edge.0].clone();
            let end = vertices[edge.1].clone();

            let delta = end.clone() - start.clone();
            // let delta_unit = delta.clone() / delta.magnitude();

            const VERTEX_DENSITY: usize = 100;
            for c in 0..VERTEX_DENSITY {
                let p = start.clone() + delta.clone() * (c as f64 / VERTEX_DENSITY as f64);
                self.write_pixel(p.0, p.1, p.2);
            }
        }
    }

    pub fn render(&mut self, shape: &shapes::Shape) {
        self.project(&shape);
        Self::clear_screen();
        for row in &self.pixels {
            for z in row {
                match z {
                    Some(p) => {
                        if *p > 10.0 {
                            print!("#");
                        } else if *p > 5.0 {
                            print!("/");
                        } else if *p > 3.0 {
                            print!("-");
                        } else {
                            print!(".");
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
