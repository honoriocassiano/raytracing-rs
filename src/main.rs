use std::io::stdout;


mod vec;
mod color;


use crate::color::Color;
use crate::color::write_color;


fn main() {

	let width: i32 = 256;
	let height: i32 = 256;

	println!("P3\n{} {}\n255", width, height);

	for line in (0..height).rev() {

		eprint!("\rScanlines remaining: {} ", line);

		for column in 0..width {
			let r: f64 = (column as f64) / ((width - 1) as f64);
			let g: f64 = (line as f64) / ((height - 1) as f64);
			let b: f64 = 0.25;

			let color = Color(r, g, b);

			write_color(&mut stdout(), &color);
		}
	}

	eprintln!("\nDone.");
}
