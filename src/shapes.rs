const VERTEX_DENSITY: usize = 100;

#[derive(Debug, Clone)]
pub struct Point(pub f64, pub f64, pub f64);

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

impl Point {
    fn set(&mut self, p: Point) {
        self.0 = p.0;
        self.1 = p.1;
        self.2 = p.2;
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn unit(&self) -> Point {
        self.clone() / self.magnitude()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn orthogonal_basis(&self) -> (Self, Self, Self) {
        // Pick vector not parallel to v
        let a = if self.0.abs() < self.1.abs() && self.0.abs() < self.2.abs() {
            Self(1.0, 0.0, 0.0)
        } else if self.1.abs() < self.2.abs() {
            Self(0.0, 1.0, 0.0)
        } else {
            Self(0.0, 0.0, 1.0)
        };

        let u = self.cross(&a).unit();
        let w = self.cross(&u).unit();
        let vecs = (self.clone(), u, w);
        dbg!(&vecs);
        vecs
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
    pub fn rotate(&mut self, pivot: &Point, (xrot, yrot, zrot): (f64, f64, f64)) {
        let (sin_xrot, cos_xrot) = xrot.sin_cos();
        let (sin_yrot, cos_yrot) = yrot.sin_cos();
        let (sin_zrot, cos_zrot) = zrot.sin_cos();
        // println!("{}", self.vertices.len());
        for v in &mut self.vertices {
            let prev = v.clone() - pivot.clone();
            let mut new = prev.clone();
            // y-rotation
            new.0 = prev.0 * cos_yrot + prev.2 * sin_yrot;
            new.2 = -prev.0 * sin_yrot + prev.2 * cos_yrot;
            let prev = new.clone();
            let mut new = prev.clone();
            // x-rotation
            new.1 = prev.1 * cos_xrot - prev.2 * sin_xrot;
            new.2 = prev.1 * sin_xrot + prev.2 * cos_xrot;
            let prev = new.clone();
            let mut new = prev.clone();
            // z-rotation
            new.0 = prev.0 * cos_zrot + prev.1 * sin_zrot;
            new.1 = -prev.0 * sin_zrot + prev.1 * cos_zrot;
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

    pub fn generate_cube(center: Point, length: f64) -> Shape {
        let mut vertices: Vec<Point> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();

        let half = length / 2.0;
        // bottom
        vertices.push(center.clone() + Point(-half, -half, -half)); // 0
        vertices.push(center.clone() + Point(-half, -half, half)); // 1
        vertices.push(center.clone() + Point(half, -half, half)); // 2
        vertices.push(center.clone() + Point(half, -half, -half)); // 3
        edges.push(Edge(0, 1));
        edges.push(Edge(1, 2));
        edges.push(Edge(2, 3));
        edges.push(Edge(0, 3));
        // top
        vertices.push(center.clone() + Point(-half, half, -half)); // 4
        vertices.push(center.clone() + Point(-half, half, half)); // 5
        vertices.push(center.clone() + Point(half, half, half)); // 6
        vertices.push(center.clone() + Point(half, half, -half)); // 7
        edges.push(Edge(4, 5));
        edges.push(Edge(5, 6));
        edges.push(Edge(6, 7));
        edges.push(Edge(4, 7));
        // left
        edges.push(Edge(0, 4));
        edges.push(Edge(1, 5));
        // right
        edges.push(Edge(2, 6));
        edges.push(Edge(3, 7));

        Shape {
            vertices,
            edges,
            center: Some(center),
        }
    }
    pub fn generate_grid(length: usize) -> Shape {
        let mut vertices: Vec<Point> = Vec::new();
        for x in 0..length {
            vertices.push(Point(-(x as f64), 0.0, 0.0));
            vertices.push(Point(x as f64, 0.0, 0.0));
        }
        for y in 0..length {
            vertices.push(Point(0.0, -(y as f64), 0.0));
            vertices.push(Point(0.0, y as f64, 0.0));
        }
        for z in 0..length {
            vertices.push(Point(0.0, 0.0, -(z as f64)));
            vertices.push(Point(0.0, 0.0, z as f64));
        }
        Shape {
            vertices,
            edges: Vec::new(),
            center: None,
        }
    }
}
