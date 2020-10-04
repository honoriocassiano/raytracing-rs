use crate::core::Ray;
use crate::core::Vec3;
use crate::hitrecord::BasicHitRecord;
use crate::material::Material;


use std::rc::Rc;


type Point3 = Vec3;


pub struct MaterialHitRecord {
	hit: BasicHitRecord,
	material: Rc<dyn Material>
}


impl MaterialHitRecord {

	pub fn new(point: Point3, t: f64, ray: Ray, outward_normal: Vec3, material: Rc<dyn Material>) -> Self {
		Self {
			hit: BasicHitRecord::new(point, t, ray, outward_normal),
			material
		}
	}

	pub fn from_hit(hit: BasicHitRecord, material: Rc<dyn Material>) -> Self {
		Self {
			hit,
			material
		}
	}

	pub fn hit(&self) -> &BasicHitRecord {
		&self.hit
	}

	pub fn point(&self) -> Point3 {
		self.hit.point()
	}

	pub fn normal(&self) -> Vec3 {
		self.hit.normal()
	}

	pub fn t(&self) -> f64 {
		self.hit.t()
	}

	pub fn front_face(&self) -> bool {
		self.hit.front_face()
	}

	pub fn material(&self) -> Rc<dyn Material> {
		return self.material.clone()
	}
}


pub trait Hit {
	fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<MaterialHitRecord>;
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
}


impl Hit for HitList {
	fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<MaterialHitRecord> {
		let mut last_hit: Option<MaterialHitRecord> = None;
		let mut closest_so_far = t_max;

		for object in &self.objects {
			match object.hit(ray, t_min, closest_so_far) {
				Some(value) => {
					closest_so_far = value.t();
					last_hit = Some(value);
				}
				None => {}
			}
		}

		last_hit
	}
}
