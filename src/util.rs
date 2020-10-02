use rand::prelude::*;


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