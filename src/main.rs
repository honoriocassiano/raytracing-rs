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
use crate::util::{PI, INFINITY, rand_between};
use crate::util::{degrees_to_radians, rand, rand_unit_vector, rand_in_hemisphere};
use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric, Material};


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


fn generate_random_scene() -> HitList {
	let mut world = HitList::new();

	let ground_material = Rc::new(Lambertian::new(&Color(0.5, 0.5, 0.5)));

	world.add(Box::new(Sphere {
		center: Point3(0.0, -1000.0, 0.0),
		radius: 1000.0,
		material: ground_material,
	}));

	for a in -11..11 {
		for b in -11..11 {
			let choose_material = rand();

			let center = Point3(
				(a as f64) + (0.9 * rand()),
				0.2,
				(b as f64) + (0.9 * rand()));

			if (center - Point3(4.0, 0.2, 0.0)).length() > 0.9 {
				let sphere_material: Rc<dyn Material>;

				if choose_material < 0.8 {
					// Diffuse
					let albedo = Color::rand() * Color::rand();

					sphere_material = Rc::new(Lambertian::new(&albedo));

				} else if choose_material < 0.95 {
					// Metal
					let albedo = Color::rand_between(0.5, 1.0);
					let fuzz = rand_between(0.0, 0.5);

					sphere_material = Rc::new(Metal::new(&albedo, fuzz));

				} else {
					// Glass
					sphere_material = Rc::new(Dielectric::new(1.5));
				}

				world.add(Box::new(Sphere {
					center,
					radius: 0.2,
					material: sphere_material,
				}));
			}
		}
	}

	let material1 = Rc::new(Dielectric::new(1.5));
	let material2 = Rc::new(Lambertian::new(&Color(0.4, 0.4, 0.1)));
	let material3 = Rc::new(Metal::new(&Color(0.7, 0.6, 0.5), 0.0));

	world.add(Box::new(Sphere {
		center: Point3(0.0, 1.0, 0.0),
		radius: 1.0,
		material: material1,
	}));

	world.add(Box::new(Sphere {
		center: Point3(-4.0, 1.0, 0.0),
		radius: 1.0,
		material: material2,
	}));

	world.add(Box::new(Sphere {
		center: Point3(4.0, 1.0, 0.0),
		radius: 1.0,
		material: material3,
	}));

	world
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
		material: material_ground.clone(),
	}));

	world.add(Box::new(Sphere {
		center: Point3(0.0, 0.0, -1.0),
		radius: 0.5,
		material: material_center.clone(),
	}));

	world.add(Box::new(Sphere {
		center: Point3(-1.0, 0.0, -1.0),
		radius: 0.5,
		material: material_left.clone(),
	}));

	// Yes, the radius is negative
	// Its don`t affect the geometry, but invert the normals
	// making the faces point to inside
	world.add(Box::new(Sphere {
		center: Point3(-1.0, 0.0, -1.0),
		radius: -0.45,
		material: material_left.clone(),
	}));

	world.add(Box::new(Sphere {
		center: Point3(1.0, 0.0, -1.0),
		radius: 0.5,
		material: material_right.clone(),
	}));

	world
}


fn main() {

	let aspect_ratio: f64 = 16.0 / 9.0;

	// let image_width: i32 = 400;
	let image_width: i32 = 1280;
	let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;

	let max_depth = 50;
	// let samples_per_pixel: i32 = 100;
	let samples_per_pixel: i32 = 500;

	let world = generate_random_scene();

	let position = Point3(13.0, 2.0, 3.0);
	let look_at = Point3(0.0, 0.0, 0.0);
	let up = Point3(0.0, 1.0, 0.0);

	let distance_to_focus = 10.0;
	let aperture = 0.1;

	let camera = Camera::new(
		position,
		look_at,
		up, 20.0, aspect_ratio, aperture, distance_to_focus);

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
