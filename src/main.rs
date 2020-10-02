use std::io::stdout;
use std::rc::Rc;


mod core;
mod material;
mod hit;
mod hitrecord;
mod sphere;
mod util;
mod camera;


use crate::core::*;
use crate::hit::{Hit, HitList};
use crate::sphere::{Sphere};
use crate::util::{PI, INFINITY};
use crate::util::{degrees_to_radians, rand, rand_unit_vector, rand_in_hemisphere};
use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric};


fn ray_color(ray: &Ray, world: &HitList, depth: i32) -> Color {

	// Stop recursion at ray bounce limit
	if depth <= 0 {
		return Color(0.0, 0.0, 0.0);
	}

	match world.hit(ray, 0.001, INFINITY) {
		Some(material_hit) => {

			match material_hit.material().scatter(ray, material_hit.hit()) {
				Some(scatter_record) => {
					scatter_record.attenuation * ray_color(&scatter_record.ray, world, depth -1)
				},
				None => {
					Color(0.0, 0.0, 0.0)
				}
			}
		},
		None => {
			let unit: Vec3 = ray.direction.normalized();

			let t: f64 = 0.5 * (unit.y() + 1.0);

			((1.0-t) * Color(1.0, 1.0, 1.0)) + (t * Color(0.5, 0.7, 1.0))
		},
	}
}



fn hit_sphere(ray: &Ray, center: &Point3, radius: f64) -> f64 {
	let oc = ray.origin - *center;

	let a: f64 = ray.direction.sq_length();
	let half_b: f64 = dot(&oc, &ray.direction);
	let c: f64 = oc.sq_length() - radius * radius;

	let discriminant: f64 = half_b*half_b - a*c;

	if discriminant < 0.0 {
		-1.0
	} else {
		(-half_b - discriminant.sqrt()) / a
	}
}


fn generate_world() -> HitList {
	let mut world = HitList::new();

	let material_ground = Rc::new(Lambertian::new(&Color(0.8, 0.8, 0.0)));
	let material_center = Rc::new(Lambertian::new(&Color(0.1, 0.2, 0.5)));
	let material_left = Rc::new(Dielectric::new(1.5));
	let material_right = Rc::new(Metal::new(&Color(0.8, 0.6, 0.2), 0.0));

	world.add(Box::new(Sphere {
		center: Point3(0.0, -100.5, -1.0),
		radius: 100.0,
		material: material_ground.clone()
	}));

	world.add(Box::new(Sphere {
		center: Point3(0.0, 0.0, -1.0),
		radius: 0.5,
		material: material_center.clone()
	}));

	world.add(Box::new(Sphere {
		center: Point3(-1.0, 0.0, -1.0),
		radius: 0.5,
		material: material_left.clone()
	}));

	world.add(Box::new(Sphere {
		center: Point3(1.0, 0.0, -1.0),
		radius: 0.5,
		material: material_right.clone()
	}));

	world
}


fn main() {

	let aspect_ratio: f64 = 16.0 / 9.0;

	let image_width: i32 = 400;
	let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;

	let max_depth = 50;
	let samples_per_pixel: i32 = 100;

	let world = generate_world();

	let camera = Camera::new();

	println!("P3\n{} {}\n255", image_width, image_height);

	for line in (0..image_height).rev() {

		eprint!("\rScanlines remaining: {} ", line);

		for column in 0..image_width {

			let mut pixel_color = Color(0.0, 0.0, 0.0);

			for sample in 0..samples_per_pixel {
				let u = (column as f64 + rand()) / (image_width - 1) as f64;
				let v = (line as f64 + rand()) / (image_height - 1) as f64;

				let ray = camera.ray(u, v);

				pixel_color += ray_color(&ray, &world, max_depth);
			}

			write_color(&mut stdout(), &pixel_color, samples_per_pixel);
		}
	}

	eprintln!("\nDone.");
}
