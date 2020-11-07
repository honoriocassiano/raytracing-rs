use std::io::Write;
use crate::util::clamp;

pub use crate::core::geometry::Vec3 as Color;


pub fn write_color<T: Write>(out: &mut T, pixel_color: Color, samples_per_pixel: i32) {
	let Color(mut r, mut g, mut b) = pixel_color;

	let scale = 1.0 / samples_per_pixel as f64;

	r = (r * scale).sqrt();
	g = (g * scale).sqrt();
	b = (b * scale).sqrt();

	let ir = (255.999 * clamp(r, 0.0, 0.999)) as i32;
	let ig = (255.999 * clamp(g, 0.0, 0.999)) as i32;
	let ib = (255.999 * clamp(b, 0.0, 0.999)) as i32;

	out.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).unwrap();
}
