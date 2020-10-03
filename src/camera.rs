use crate::core::Vec3;
use crate::core::{Ray, Point3};
use crate::util::degrees_to_radians;


pub struct Camera {
	position: Point3,
	lower_left_corner: Point3,

	horizontal: Vec3,
	vertical: Vec3
}


impl Camera {

	pub fn new(vertical_fov: f64, aspect_ratio: f64) -> Self {

		let theta: f64 = degrees_to_radians(vertical_fov);
		let h: f64 = (theta/2.0).tan();

		let viewport_height: f64 = 2.0 * h;
		let viewport_width: f64 = aspect_ratio * viewport_height;

		let focal_length: f64 = 1.0;

		let position = Point3(0.0, 0.0, 0.0);
		let horizontal = Vec3(viewport_width, 0.0, 0.0);
		let vertical = Vec3(0.0, viewport_height, 0.0);

		let lower_left_corner = position - horizontal/2.0 - vertical/2.0 - Vec3(0.0, 0.0, focal_length);

		Self {
			position,
			lower_left_corner,
			horizontal,
			vertical
		}
	}
	
	pub fn ray(&self, u: f64, v: f64) -> Ray {
		Ray {
			origin: self.position,
			direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.position
		}
	}
}
