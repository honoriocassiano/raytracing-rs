pub use crate::core::geometry::vec3::Vec3 as Point3;

type Scalar = f64;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn at(&self, t: Scalar) -> Point3 {
        self.origin + (self.direction * t)
    }
}
