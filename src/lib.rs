#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64, pub f64);

pub mod shapes;

impl Point {
    fn set(&mut self, p: Point) {
        self.0 = p.0;
        self.1 = p.1;
        self.2 = p.2;
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, a: Self) -> Self {
        Self(self.0 + a.0, self.1 + a.1, self.2 + a.2)
    }
}
impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, a: Self) -> Self {
        Self(self.0 - a.0, self.1 - a.1, self.2 - a.2)
    }
}

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

    fn project(&mut self, shape: &shapes::Shape) {
        self.pixels = vec![vec![None; self.x_size]; self.y_size];
        let vertices = &shape.vertices;

        for v in vertices {
            let x = (v.0 + (self.x_size as f64) / 2.0) as isize;
            let y = (v.1 + (self.y_size as f64) / 2.0) as isize;
            if (x < 0) || (y < 0) {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if (x >= self.x_size) || (y >= self.y_size) {
                continue;
            }
            let z = v.2;
            let cur_z = self.pixels[y][x];
            if let Some(k) = cur_z {
                if z > k {
                    self.pixels[y][x] = Some(z);
                }
            } else {
                self.pixels[y][x] = Some(z);
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
