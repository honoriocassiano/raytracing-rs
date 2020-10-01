use crate::vec::{dot};
use crate::ray::{Point3, Ray};
use crate::hit::{Hit, HitRecord};

pub struct Sphere {
	pub center: Point3,
	pub radius: f64
}

impl Hit for Sphere {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

		let oc = ray.origin - self.center;

		let a: f64 = ray.direction.sq_length();
		let half_b: f64 = dot(&oc, &ray.direction);
		let c: f64 = oc.sq_length() - self.radius * self.radius;

		let discriminant: f64 = half_b*half_b - a*c;

		if discriminant > 0.0 {
			let root: f64 = discriminant.sqrt();

			let mut temp: f64 = (-half_b - root) / a;

			if (t_min < temp) && (temp < t_max) {

				let t = temp;
				let point = ray.at(temp);

				let outward_normal = (point - self.center) / self.radius;

				return Some(HitRecord::new(&point, t, ray, &outward_normal));

				// let normal = (point - self.center) / self.radius;

				// return Some(HitRecord::new(point, normal, t,front_face: false);
			}

			temp = (-half_b + root) / a;

			if (t_min < temp) && (temp < t_max) {
				let t = temp;
				let point = ray.at(temp);

				let outward_normal = (point - self.center) / self.radius;

				return Some(HitRecord::new(&point, t, ray, &outward_normal));
				
				// let normal = (point - self.center) / self.radius;

				// return Some(HitRecord {
				// 	point,
				// 	normal,
				// 	t,
				// 	front_face: false
				// });
			}
		}

		None
	}
}