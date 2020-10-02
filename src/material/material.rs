use crate::core::Color;
use crate::core::Ray;
use crate::hitrecord::BasicHitRecord;


pub struct ScatterRecord {
	pub attenuation: Color,
	pub ray: Ray
}


pub trait Material {
	fn scatter(&self, in_ray: &Ray, hit: &BasicHitRecord) -> Option<ScatterRecord>;
}
