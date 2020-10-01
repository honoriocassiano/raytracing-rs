use std::io::stdout;


mod vec;
mod ray;
mod color;


use crate::vec::Vec3;
use crate::vec::{dot, cross};
use crate::color::Color;
use crate::color::write_color;
use crate::ray::{Ray, Point3};


fn ray_color(ray: &Ray) -> Color {

	if hit_sphere(ray, &Point3(0.0, 0.0, -1.0), 0.5) {

		Color(1.0, 0.0, 0.0)

	} else {

		let unit: Vec3 = ray.direction.normalized();

		let t: f64 = 0.5 * (unit.y() + 1.0);

		((1.0-t) * Color(1.0, 1.0, 1.0)) + (t * Color(0.5, 0.7, 1.0))
	}
}


fn hit_sphere(ray: &Ray, center: &Point3, radius: f64) -> bool {
	let oc = ray.origin - *center;

	let a: f64 = dot(&ray.direction, &ray.direction);
	let b: f64 = 2.0 * dot(&oc, &ray.direction);
	let c: f64 = dot(&oc, &oc) - radius * radius;

	let discriminant: f64 = b*b - 4.0*a*c;

	discriminant > 0.0
}


fn main() {

	let aspect_ratio: f64 = 16.0 / 9.0;

	let image_width: i32 = 400;
	let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;

	let viewport_height: f64 = 2.0;
	let viewport_width: f64 = aspect_ratio * viewport_height;
	let focal_length: f64 = 1.0;

	let origin = Point3(0.0, 0.0, 0.0);
	let horizontal = Vec3(viewport_width, 0.0, 0.0);
	let vertical = Vec3(0.0, viewport_height, 0.0);

	let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3(0.0, 0.0, focal_length);

	println!("P3\n{} {}\n255", image_width, image_height);

	for line in (0..image_height).rev() {

		eprint!("\rScanlines remaining: {} ", line);

		for column in 0..image_width {
			let u = (column as f64) / (image_width - 1) as f64;
			let v = (line as f64) / (image_height - 1) as f64;

			let r = Ray{
				origin,
				direction: lower_left_corner + u*horizontal + v*vertical - origin
			};


			let color = ray_color(&r);

			write_color(&mut stdout(), &color);
		}
	}

	eprintln!("\nDone.");
}
