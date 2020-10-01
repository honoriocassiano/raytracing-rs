use std::io::Write;


pub use crate::vec::Vec3 as Color;


pub fn write_color<T: Write>(out: &mut T , color: &Color) {

	let ir = (255.999 * color.x()) as i32;
	let ig = (255.999 * color.y()) as i32;
	let ib = (255.999 * color.z()) as i32;

	out.write(format!("{} {} {}", ir, ig, ib).as_bytes()).unwrap();
}
