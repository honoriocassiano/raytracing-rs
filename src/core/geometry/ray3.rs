use super::{Point3, Ray, Vec3};

#[derive(Copy, Clone)]
pub struct Ray3 {
    origin: Point3,
    direction: Vec3,
}

impl Ray3 {
    pub fn new(origin: Point3, direction: Vec3) -> Ray3 {
        Ray3 { origin, direction }
    }
}

impl Ray for Ray3 {
    type Point = Point3;
    type Vector = Vec3;
    type Scalar = f64;

    fn at(&self, t: f64) -> Self::Point {
        self.origin + (self.direction * t)
    }

    fn origin(self) -> Self::Point {
        self.origin
    }

    fn direction(self) -> Self::Vector {
        self.direction
    }
}
