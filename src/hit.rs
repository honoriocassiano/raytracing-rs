use crate::ray::Ray;
use crate::vec::Vec3;

use crate::vec::Vec3 as Point3;

pub struct HitRecord {
	pub point: Point3,
	pub normal: Vec3,
	pub t: f64
}

pub trait Hit {
	fn hit(ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}