use std::io::Write;
use crate::util::clamp;

pub use crate::vec::Vec3 as Color;


pub fn write_color<T: Write>(out: &mut T, pixel_color: &Color, samples_per_pixel: i32) {

	let mut color: Color = *pixel_color;

	let scale = 1.0 / samples_per_pixel as f64;

	color *= scale;

	let ir = (255.999 * clamp(color.x(), 0.0, 0.999)) as i32;
	let ig = (255.999 * clamp(color.y(), 0.0, 0.999)) as i32;
	let ib = (255.999 * clamp(color.z(), 0.0, 0.999)) as i32;

	out.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()).unwrap();
}
