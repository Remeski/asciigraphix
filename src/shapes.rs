const VERTEX_DENSITY: usize = 100;

#[derive(Debug, Clone)]
pub struct Point(pub f64, pub f64, pub f64);

impl Point {
    fn set(&mut self, p: Point) {
        self.0 = p.0;
        self.1 = p.1;
        self.2 = p.2;
    }
    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

#[derive(Debug, Clone)]
pub struct Edge(pub usize, pub usize);

#[derive(Debug)]
pub struct Shape {
    pub vertices: Vec<Point>,
    pub edges: Vec<Edge>,
    pub center: Option<Point>,
}

impl Shape {
    pub fn rotate(&mut self, pivot: &Point, (theta, phi): (f64, f64)) {
        let (sin_theta, cos_theta) = theta.sin_cos();
        let (sin_phi, cos_phi) = phi.sin_cos();
        // println!("{}", self.vertices.len());
        for v in &mut self.vertices {
            let prev = v.clone() - pivot.clone();
            let mut new = prev.clone();
            new.0 = prev.0 * cos_theta + prev.2 * sin_theta;
            new.2 = -prev.0 * sin_theta + prev.2 * cos_theta;
            let prev = new.clone();
            let mut new = prev.clone();
            new.1 = prev.1 * cos_phi - prev.2 * sin_phi;
            new.2 = prev.1 * sin_phi + prev.2 * cos_phi;
            v.set(new + pivot.clone());
        }
    }
    pub fn combine(&self, s2: &Self) -> Self {
        let mut vertices = self.vertices.clone();
        let mut edges = self.edges.clone();
        vertices.append(&mut s2.vertices.clone());
        edges.append(&mut s2.edges.clone());
        Shape {
            vertices,
            edges,
            center: None,
        }
    }

    pub fn generate_ring(radius: f64, center: Point) -> Shape {
        let mut vertices: Vec<Point> = Vec::new();

        for c in 0..VERTEX_DENSITY {
            // < rcos(t), rsin(t), 0.0 >
            let (s, c) = (2.0 * 3.14 / VERTEX_DENSITY as f64 * c as f64).sin_cos();
            let point = Point(radius * s + center.0, radius * c + center.1, 0.0 + center.2);
            vertices.push(point);
        }
        Shape {
            vertices,
            edges: Vec::new(),
            center: Some(center),
        }
    }

    pub fn generate_line(start: Point, end: Point) -> Shape {
        Shape {
            vertices: vec![start, end],
            edges: vec![Edge(0, 1)],
            center: None,
        }
    }
}
