use rand::prelude::*;
use crate::core::{Vec3};


pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;


pub fn degrees_to_radians(deg: f64) -> f64 {
	deg * PI / 180.0
}


pub fn rand() -> f64 {
	let mut rng = rand::thread_rng();

	rng.gen_range(0.0, 1.0)
}


pub fn rand_between(min: f64, max: f64) -> f64 {
	let mut rng = rand::thread_rng();

	rng.gen_range(min, max)
}


pub fn clamp(value: f64, min: f64, max: f64) -> f64 {

	if value < min {
		min
	} else if value > max {
		max
	} else {
		value
	}
}



pub fn rand_unit_vector() -> Vec3 {
	let azimuth = rand_between(0.0, 2.0*PI);
	let z = rand_between(-1.0, 1.0);
	let radius = (1.0 - z*z).sqrt();

	Vec3(radius*azimuth.cos(), radius*azimuth.sin(), z)
}


pub fn rand_in_hemisphere(normal: &Vec3) -> Vec3 {
	let unit_in_shpere = rand_unit_vector();

	let same_hemispherial = unit_in_shpere.dot(*normal) > 0.0;

	match same_hemispherial {
		true => unit_in_shpere,
		false => -unit_in_shpere
	}
}


pub fn rand_in_unit_disk() -> Vec3 {
	loop {
		let p = Vec3(rand_between(-1.0, 1.0), rand_between(-1.0, 1.0), 0.0);

		if p.sq_length() < 1.0 {
			break p;
		}
	}
}
