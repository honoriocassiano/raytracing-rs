pub use crate::vec::Vec3 as Point3;

type Scalar = f64;

pub struct Ray {
	pub origin: Point3,
	pub direction: Point3
}

impl Ray {
	
	pub fn at(&self, t: Scalar) -> Point3 {
		self.origin + (self.direction * t)
	}
}