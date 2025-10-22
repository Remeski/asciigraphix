const VERTEX_DENSITY: usize = 100;

// Really whis is a 3D vector...
#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64, pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Point4(pub f64, pub f64, pub f64, pub f64);

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Add for Point4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Sub for Point4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl std::ops::Div<f64> for Point4 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Mul<f64> for Point4 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
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

impl Point4 {
    pub fn set(&mut self, p: Point4) {
        self.0 = p.0;
        self.1 = p.1;
        self.2 = p.2;
        self.3 = p.3;
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 + self.3 * rhs.3
    }

    pub fn unit(&self) -> Point4 {
        self.clone() / self.magnitude()
    }
}

// This holds the indexes of two points forming an edge.
#[derive(Debug, Clone)]
pub struct Edge(pub usize, pub usize);

// This holds the indexes of points forming a face.
#[derive(Debug, Clone)]
pub struct Face(pub usize, pub usize, pub usize);

#[derive(Debug)]
pub struct Shape {
    pub vertices: Vec<Point>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub center: Option<Point>,
}

#[derive(Debug)]
pub struct Shape4 {
    pub vertices: Vec<Point4>,
    pub edges: Vec<Edge>,
    pub center: Option<Point4>,
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
        let mut faces = self.faces.clone();
        vertices.append(&mut s2.vertices.clone());
        edges.append(&mut s2.edges.clone());
        faces.append(&mut s2.faces.clone());
        Shape {
            vertices,
            edges,
            faces,
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
            faces: Vec::new(),
            center: Some(center),
        }
    }

    pub fn generate_line(start: Point, end: Point) -> Shape {
        Shape {
            vertices: vec![start, end],
            edges: vec![Edge(0, 1)],
            faces: Vec::new(),
            center: None,
        }
    }

    pub fn generate_cube(center: Point, length: f64) -> Shape {
        let mut vertices: Vec<Point> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

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

        faces.push(Face(0, 1, 2));

        Shape {
            vertices,
            edges,
            faces,
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
            faces: Vec::new(),
            center: None,
        }
    }

    pub fn generate_parallelepiped(start: Point, a: Point, b: Point, c: Point) -> Shape {
        let mut vertices: Vec<Point> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();
        vertices.push(start); // 0
        vertices.push(start + a); // 1
        edges.push(Edge(0, 1));

        vertices.push(start + c); // 2
        vertices.push(start + c + a); // 3
        edges.push(Edge(2, 3));
        edges.push(Edge(0, 2));
        edges.push(Edge(1, 3));

        vertices.push(start + b); // 4
        vertices.push(start + b + a); // 5
        edges.push(Edge(4, 5));

        vertices.push(start + b + c); // 6
        vertices.push(start + b + c + a); // 7
        edges.push(Edge(6, 7));
        edges.push(Edge(4, 6));
        edges.push(Edge(5, 7));

        edges.push(Edge(0, 4));
        edges.push(Edge(1, 5));
        edges.push(Edge(2, 6));
        edges.push(Edge(3, 7));

        Shape {
            vertices,
            edges,
            faces: Vec::new(),
            center: Some(start),
        }
    }
}

impl Shape4 {
    pub fn generate_4d_paralellepiped(
        start: Point4,
        a: Point4,
        b: Point4,
        c: Point4,
        d: Point4,
    ) -> Shape4 {
        let mut vertices: Vec<Point4> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();

        vertices.push(start); // 0
        vertices.push(start + a); // 1
        vertices.push(start + b); // 2
        vertices.push(start + c); // 3
        vertices.push(start + d); // 4
        edges.push(Edge(0, 1));
        edges.push(Edge(0, 2));
        edges.push(Edge(0, 3));
        edges.push(Edge(0, 4));

        vertices.push(start + a + d); // 5
        vertices.push(start + a + b); // 6
        vertices.push(start + a + c); // 7
        vertices.push(start + b + c); // 8
        vertices.push(start + b + d); // 9
        vertices.push(start + c + d); // 10
        edges.push(Edge(1, 5)); // a to a + d
        edges.push(Edge(1, 6)); // a to a + b
        edges.push(Edge(1, 7)); // a to a + c
        edges.push(Edge(2, 6)); // b to a + b
        edges.push(Edge(2, 8)); // b to b + c
        edges.push(Edge(2, 9)); // b to b + d
        edges.push(Edge(3, 7)); // c to a + c
        edges.push(Edge(3, 8)); // c to b + c
        edges.push(Edge(3, 10)); // c to c + d
        edges.push(Edge(4, 5)); // d to a + d
        edges.push(Edge(4, 9)); // d to b + d
        edges.push(Edge(4, 10)); // d to c + d

        vertices.push(start + a + b + c); // 11
        vertices.push(start + a + b + d); // 12
        vertices.push(start + a + c + d); // 13
        vertices.push(start + b + c + d); // 14
        edges.push(Edge(5, 12)); // a + d to a + b + d
        edges.push(Edge(5, 13)); // a + d to a + c + d
        edges.push(Edge(6, 11)); // a + b to a + b + c
        edges.push(Edge(6, 12)); // a + b to a + b + d
        edges.push(Edge(7, 11)); // a + c to a + b + c
        edges.push(Edge(7, 14)); // a + c to b + c + d
        edges.push(Edge(8, 11)); // b + c to a + b + c
        edges.push(Edge(8, 14)); // b + c to b + c + d
        edges.push(Edge(9, 12)); // b + d to a + b + d
        edges.push(Edge(9, 14)); // b + d to b + c + d
        edges.push(Edge(10, 13)); // c + d to a + c + d
        edges.push(Edge(10, 14)); // c + d to b + c + d

        vertices.push(start + a + b + c + d); // 15
        edges.push(Edge(11,15)); // a + b + c to a + b + c + d
        edges.push(Edge(12,15)); // a + b + d to a + b + c + d
        edges.push(Edge(13,15)); // a + c + d to a + b + c + d
        edges.push(Edge(14,15)); // b + c + d to a + b + c + d

        Shape4 {
            vertices,
            edges,
            center: None,
        }
    }

    pub fn rotate(
        &mut self,
        pivot: &Point4,
        (xyrot, yzrot, xzrot, wxrot, wyrot, wzrot): (f64, f64, f64, f64, f64, f64),
    ) {
        let (sin_xyrot, cos_xyrot) = xyrot.sin_cos();
        let (sin_yzrot, cos_yzrot) = yzrot.sin_cos();
        let (sin_xzrot, cos_xzrot) = xzrot.sin_cos();
        let (sin_wxrot, cos_wxrot) = wxrot.sin_cos();
        let (sin_wyrot, cos_wyrot) = wyrot.sin_cos();
        let (sin_wzrot, cos_wzrot) = wzrot.sin_cos();
        // println!("{}", self.vertices.len());
        for v in &mut self.vertices {
            let prev = v.clone() - pivot.clone();
            let mut new = prev.clone();
            // xz-rotation
            new.0 = prev.0 * cos_xzrot + prev.2 * sin_xzrot;
            new.2 = -prev.0 * sin_xzrot + prev.2 * cos_xzrot;
            let prev = new.clone();
            let mut new = prev.clone();
            // yz-rotation
            new.1 = prev.1 * cos_yzrot - prev.2 * sin_yzrot;
            new.2 = prev.1 * sin_yzrot + prev.2 * cos_yzrot;
            let prev = new.clone();
            let mut new = prev.clone();
            // xy-rotation
            new.0 = prev.0 * cos_xyrot + prev.1 * sin_xyrot;
            new.1 = -prev.0 * sin_xyrot + prev.1 * cos_xyrot;
            let prev = new.clone();
            let mut new = prev.clone();
            // wx-rotation
            new.0 = prev.0 * cos_wxrot + prev.3 * sin_wxrot;
            new.3 = -prev.0 * sin_wxrot + prev.3 * cos_wxrot;
            let prev = new.clone();
            let mut new = prev.clone();
            // wy-rotation
            new.1 = prev.1 * cos_wyrot + prev.3 * sin_wyrot;
            new.3 = -prev.1 * sin_wyrot + prev.3 * cos_wyrot;
            let prev = new.clone();
            let mut new = prev.clone();
            // wz-rotation
            new.2 = prev.2 * cos_wzrot + prev.3 * sin_wzrot;
            new.3 = -prev.2 * sin_wzrot + prev.3 * cos_wzrot;
            v.set(new + pivot.clone());
        }
    }

    pub fn project_to_3d(&self) -> Shape {
        let mut vertices: Vec<Point> = Vec::new();

        for v in &self.vertices {
            vertices.push(Point(v.0, v.1, v.2))
        }

        Shape {
            vertices,
            edges: self.edges.clone(),
            faces: Vec::new(),
            center: None,
        }
    }
}
