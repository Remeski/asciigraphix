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

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Point {
    pub fn e(i: usize) -> Self {
        match i {
            1 => Self(1.0, 0.0, 0.0),
            2 => Self(0.0, 1.0, 0.0),
            3 => Self(0.0, 0.0, 1.0),
            _ => {
                panic!("invalid e vector")
            }
        }
    }

    pub fn zero() -> Self {
        Point(0.0, 0.0, 0.0)
    }

    /// Returns barycentric coordinates within triangle with vertices `v1`, `v2`, `v3`.
    pub fn to_barycentric(&self, v1: Point, v2: Point, v3: Point) -> Point {
        let v12 = v2 - v1;
        let v13 = v3 - v1;
        let v1p = *self - v1;

        let v21_inv = 1.0 / v12.dot(&v12);
        let w = v1p - v12 * (v1p.dot(&v12) * v21_inv);
        let v = v13 - v12 * (v13.dot(&v12) * v21_inv);

        let m3 = v.dot(&w) / v.dot(&v);
        let m2 = (v1p - v13 * m3).dot(&v12) * v21_inv;
        let m1 = 1.0 - m2 - m3;

        Point(m1, m2, m3)
    }

    pub fn set(&mut self, p: Point) {
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

    // perhaps consider making some matrix multiplication feature
    pub fn rotate(&mut self, theta: f64, phi: f64) {
        // i' = cos theta i + sin theta j
        // j' = -sin theta i + cos theta j
        //
        // cos theta -sin theta
        // sin theta cos theta
        let old_0 = self.0.clone();
        let old_1 = self.1.clone();
        self.0 = theta.cos() * old_0 - theta.sin() * old_1;
        self.1 = theta.sin() * old_0 + theta.cos() * old_1;
        // let old_0 = self.0.clone();
        // let old_1 = self.1.clone();
        // self.0 = phi.cos() * old_0 - phi.sin() * old_1;
        // self.1 = phi.sin() * old_0 + phi.cos() * old_1;

        // rotate in c = (-y, x, 0)/r,p = (x,y,0)/r ,k = (0,0,1) basis where coordinates c = 0, p = r, k = z
        let r = (self.0.powf(2.0) + self.1.powf(2.0)).sqrt();
        let z = self.2.clone();
        let p_new = phi.cos() * r - phi.sin() * z;
        let k_new = phi.sin() * r + phi.cos() * z;
        // to natural basis
        let old_0 = self.0.clone();
        let old_1 = self.1.clone();
        // let old_2 = self.2;
        self.0 = (old_0 * p_new) / r;
        self.1 = (old_1 * p_new) / r;
        self.2 = k_new;
    }
}

impl Point4 {
    pub fn e(i: usize) -> Self {
        match i {
            1 => Self(1.0, 0.0, 0.0, 0.0),
            2 => Self(0.0, 1.0, 0.0, 0.0),
            3 => Self(0.0, 0.0, 1.0, 0.0),
            4 => Self(0.0, 0.0, 0.0, 1.0),
            _ => {
                panic!("invalid e vector")
            }
        }
    }

    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0, 0.0)
    }

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

// 2D: X = u1 + t(u2-u1) = v1 + h(v2-v1)
// => cross(u1,v2-v1) + t cross(u2-u1, v2-v1) = cross(v1, v2-v1) 
// => t = cross(v1 - u1, v2 - v1).2/cross(u2-u1, v2-v1).2
// likewise for h
//
// 3D: X = u1 + t(u2-u1) = v1 + h(v2-v1)
// => u1 - v1 = h(v2-v1) - t(u2-u1) = ((v2-v1)  -(u2-u1)) (h, t)
// => (h, t) = ((v2-v1)  -(u2-u1))^-1 (u1-v1)
//          = Q^-1 (u1-v1)
pub fn line_intersection((u1, u2): (Point, Point), (v1, v2): (Point, Point)) -> Option<Point> {
    let u = u2 - u1;
    let v = v2 - v1;
    let uv = v1 - u1;
    if u1.2 == 0.0 && u2.2 == 0.0 && v1.2 == 0.0 && v2.2 == 0.0 {
        let t = uv.cross(&v).2 / u.cross(&v).2;
        let h = -uv.cross(&u).2 / v.cross(&u).2;
        if t >= 0.0 && t <= 1.0 && h >= 0.0 && h <= 1.0 {
            Some(u1 + u * t)
        } else {
            None
        }
    } else {
        let q00 = u.dot(&u);
        let q01 = -u.dot(&v);
        let q11 = v.dot(&v);

        let u_uv = u.dot(&uv);
        let v_uv = -v.dot(&uv);

        let det = q00 * q11 - q01 * q01;

        if det.abs() < 1e-3 {
            return None;
        }

        let det_inv = 1.0 / det;
        let h = det_inv * (u_uv * q11 - v_uv * q01);
        let t = det_inv * (-u_uv * q01 + v_uv * q00);

        dbg!(h, t);

        let u_point = u1 + u * h;
        let v_point = v1 + v * t;
        if (u_point - v_point).magnitude() < 1e-3 {
            Some(u1 + u * h)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Point, line_intersection};

    #[test]
    fn test_barycentric() {
        let v1 = Point(0.0, 0.0, 0.0);
        let v2 = Point(1.0, 0.0, 0.0);
        let v3 = Point(0.0, 1.0, 0.0);
        let p = Point(0.25, 0.25, 0.0);
        let bary = p.to_barycentric(v1, v2, v3);
        assert_eq!(bary, Point(0.5, 0.25, 0.25));
        assert_eq!(bary.0 + bary.1 + bary.2, 1.0);
        assert_eq!(v1 * bary.0 + v2 * bary.1 + v3 * bary.2, p);
    }

    #[test]
    fn test_line_intersections() {
        let v1 = Point(0.0, 0.0, 0.0);
        let v2 = Point(5.0, 0.5, 0.0);

        let u1 = Point(0.0, -1.0, 0.0);
        let u2 = Point(5.0, 1.0, 0.0);

        let inter = line_intersection((v1, v2), (u1, u2));
        dbg!(inter);

        assert!(inter.is_none());
    }
}
