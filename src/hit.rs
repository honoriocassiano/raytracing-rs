use crate::ray::Ray;
use crate::vec::{Vec3, dot};


use crate::vec::Vec3 as Point3;


pub struct HitRecord {
	point: Point3,
	normal: Vec3,
	t: f64,
	front_face: bool
}


impl HitRecord {

	pub fn new(point: &Point3, t: f64, ray: &Ray, outward_normal: &Vec3) -> Self {

		let front_face = dot(&ray.direction, &outward_normal) < 0.0;
		let normal = if front_face {*outward_normal} else {-*outward_normal};

		Self {
			point: *point,
			normal,
			front_face,
			t
		}
	}

	pub fn point(&self) -> Point3 {
		self.point
	}

	pub fn normal(&self) -> Vec3 {
		self.normal
	}

	pub fn t(&self) -> f64 {
		self.t
	}

	pub fn front_face(&self) -> bool {
		self.front_face
	}
}


pub trait Hit {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}