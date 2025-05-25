use crate::Point;
const VERTEX_DENSITY: usize = 100;

#[derive(Debug)]
pub struct Shape {
    pub vertices: Vec<Point>,
    pub center: Option<Point>,
}

impl Shape {
    pub fn rotate(&mut self, pivot: &Point, (theta, phi): (f64, f64)) {
        let (sin_theta, cos_theta) = theta.sin_cos();
        let (sin_phi, cos_phi) = phi.sin_cos();
        // println!("{}", self.vertices.len());
        for v in &mut self.vertices {
            let prev = *v - *pivot;
            let mut new = prev;
            new.0 = prev.0 * cos_theta + prev.2 * sin_theta;
            new.2 = -prev.0 * sin_theta + prev.2 * cos_theta;
            let prev = new;
            let mut new = prev;
            new.1 = prev.1 * cos_phi - prev.2 * sin_phi;
            new.2 = prev.1 * sin_phi + prev.2 * cos_phi;
            v.set(new + *pivot);
        }
    }
    pub fn combine(&self, s2: &Self) -> Self {
        let mut vertices = self.vertices.clone();
        vertices.append(&mut s2.vertices.clone());
        Shape {
            vertices,
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
            center: Some(center),
        }
    }
}
