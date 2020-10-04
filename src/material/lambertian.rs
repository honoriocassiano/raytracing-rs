use crate::core::{Color, Ray};
use crate::util::rand_unit_vector;
use crate::hitrecord::BasicHitRecord;

use super::material::{Material, ScatterRecord};


pub struct Lambertian {
	albedo: Color
}

impl Lambertian {
	pub fn new(color: Color) -> Self {
		Self {
			albedo: color
		}
	}

	pub fn albedo(&self) -> Color {
		self.albedo
	}
}


impl Material for Lambertian {
	fn scatter(&self, in_ray: &Ray, hit: &BasicHitRecord) -> Option<ScatterRecord> {
		let scatter_direction = hit.normal() + rand_unit_vector();

		let scatter_record = ScatterRecord{
			ray: Ray { origin: hit.point(), direction: scatter_direction },
			attenuation: self.albedo
		};

		Some(scatter_record)
	}
}