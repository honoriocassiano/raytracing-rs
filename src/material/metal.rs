use crate::core::{Color, Ray};
use crate::util::rand_unit_vector;
use crate::hitrecord::BasicHitRecord;

use super::material::{Material, ScatterRecord};


pub struct Metal {
	albedo: Color,
	fuzz: f64
}

impl Metal {
	pub fn new(color: Color, fuzz: f64) -> Self {
		Self {
			albedo: color,
			fuzz: fuzz.min(1.0) // TODO Check if this value can be negative
		}
	}

	pub fn albedo(&self) -> Color {
		self.albedo
	}

	pub fn fuzz(&self) -> f64 {
		self.fuzz
	}
}


impl Material for Metal {
	fn scatter(&self, in_ray: &Ray, hit: &BasicHitRecord) -> Option<ScatterRecord> {
		let reflected = in_ray.direction.normalized().reflect(hit.normal());

		let scatter_record = ScatterRecord{
			ray: Ray {
				origin: hit.point(),
				direction: reflected + self.fuzz * rand_unit_vector()
			},
			attenuation: self.albedo
		};


		Some(scatter_record)
	}
}