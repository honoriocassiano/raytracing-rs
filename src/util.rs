
pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(deg: f64) -> f64 {
	deg * PI / 180.0
}
