use crate::core::{Color, Vec3, Ray, dot};
use crate::util::{rand_unit_vector};
use crate::hitrecord::BasicHitRecord;

use super::material::{Material, ScatterRecord};


pub struct Dielectric {
	refractive_index: f64
}

impl Dielectric {
	pub fn new(refractive_index: f64) -> Self {
		Self {
			refractive_index
		}
	}

	pub fn refractive_index(&self) -> f64 {
		self.refractive_index
	}
}


impl Material for Dielectric {
	fn scatter(&self, in_ray: &Ray, hit: &BasicHitRecord) -> Option<ScatterRecord> {
		let eta_in_over_eta_out = {
			if hit.front_face() {
				1.0 / self.refractive_index
			} else {
				self.refractive_index
			}
		};

		let unit_direction = in_ray.direction.normalized();

		let cos_theta = dot(&(-unit_direction), &hit.normal()).min(1.0);
		let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

		let scatter_direction: Vec3 = {
			if (eta_in_over_eta_out * sin_theta) > 1.0 {
				unit_direction.reflect(&hit.normal())
			} else {
				unit_direction.refract(&hit.normal(), eta_in_over_eta_out)
			}
		};

		let scatter_record = ScatterRecord {
			ray: Ray { origin: hit.point(), direction: scatter_direction },
			attenuation: Color(1.0, 1.0, 1.0),
		};

		Some(scatter_record)
	}
}