use crate::core::{Point3, Ray};
use crate::materials::Material;
use crate::hit::{Hit, MaterialHitRecord};


use std::rc::Rc;


pub struct Sphere {
	pub center: Point3,
	pub radius: f64,
	pub material: Rc<dyn Material>
}


impl Hit for Sphere {
	fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<MaterialHitRecord> {

		let oc = ray.origin - self.center;

		let a: f64 = ray.direction.sq_length();
		let half_b: f64 = oc.dot(ray.direction);
		let c: f64 = oc.sq_length() - self.radius * self.radius;

		let discriminant: f64 = half_b*half_b - a*c;

		if discriminant > 0.0 {
			let root: f64 = discriminant.sqrt();

			let mut temp: f64 = (-half_b - root) / a;

			if (t_min < temp) && (temp < t_max) {

				let t = temp;
				let point = ray.at(temp);

				let outward_normal = (point - self.center) / self.radius;

				return Some(MaterialHitRecord::new(point, t, ray, outward_normal, self.material.clone()));
			}

			temp = (-half_b + root) / a;

			if (t_min < temp) && (temp < t_max) {
				let t = temp;
				let point = ray.at(temp);

				let outward_normal = (point - self.center) / self.radius;

				return Some(MaterialHitRecord::new(point, t, ray, outward_normal, self.material.clone()));
			}
		}

		None
	}
}
