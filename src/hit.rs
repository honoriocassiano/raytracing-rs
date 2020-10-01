use crate::ray::Ray;
use crate::vec::{Vec3, dot};


use crate::vec::Vec3 as Point3;


#[derive(Copy, Clone)]
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


// Hit list
pub struct HitList {
	objects: Vec<Box<(dyn Hit)>>
}


impl HitList {
	pub fn new() -> Self {
		Self { objects: Vec::new() }
	}

	pub fn with(object: Box<dyn Hit>) -> Self {
		Self { objects: vec!(object) }
	}

	pub fn clear(&mut self) {
		self.objects.clear();
	}

	pub fn add(&mut self, object: Box<dyn Hit>) {
		self.objects.push(object);
	}

	// pub fn add() {

	// }
}


impl Hit for HitList {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut last_hit: Option<HitRecord> = None;
		let mut closest_so_far = t_max;

		for object in &self.objects {
			match object.hit(ray, t_min, closest_so_far) {
				Some(value) => {
					last_hit = Some(value);
					closest_so_far = value.t();
				}
				None => {}
			}
		}

		last_hit
	}
}