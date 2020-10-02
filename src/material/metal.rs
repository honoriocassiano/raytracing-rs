use crate::core::{Vec3, Color, Ray};
use crate::util::{rand_unit_vector};
use crate::hitrecord::BasicHitRecord;

use super::material::{Material, ScatterRecord};


pub struct Metal {
	albedo: Color
}

impl Metal {
	pub fn new(color: &Color) -> Self {
		Self {
			albedo: *color
		}
	}

	pub fn albedo(&self) -> Color {
		self.albedo
	}
}


impl Material for Metal {
	fn scatter(&self, in_ray: &Ray, hit: &BasicHitRecord) -> Option<ScatterRecord> {
		let reflected = in_ray.direction.normalized().reflect(&hit.normal());

		let scatter_record = ScatterRecord{
			ray: Ray { origin: hit.point(), direction: reflected },
			attenuation: self.albedo
		};


		Some(scatter_record)
	}
}