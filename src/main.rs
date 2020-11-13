use std::io::stdout;
use std::rc::Rc;

mod core;
mod materials;
mod scene;
mod textures;

use self::core::math::constants::INFINITY;
use crate::core::color::write_color;
use crate::core::color::Color;
use crate::core::geometry::{Point3, Ray, Vec3, Vector};
use crate::core::math::rand::{rand, rand_between};
use crate::core::time::{Interval, TimeRay3};
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::scene::camera::Options;
use crate::scene::object::movingsphere::MovingSphere;
use crate::scene::{Hit, HitList};
use scene::camera::Camera;
use scene::object::sphere::Sphere;

fn ray_color(ray: TimeRay3, world: &HitList, depth: i32) -> Color {
    // Stop recursion at ray bounce limit
    if depth <= 0 {
        return Color(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, INFINITY) {
        Some(material_hit) => match material_hit.material().scatter(ray, material_hit.hit()) {
            Some(scatter_record) => {
                scatter_record.attenuation * ray_color(scatter_record.ray, world, depth - 1)
            }
            None => Color(0.0, 0.0, 0.0),
        },
        None => {
            let unit: Vec3 = ray.direction().normalized();

            let t: f64 = 0.5 * (unit.y() + 1.0);

            ((1.0 - t) * Color(1.0, 1.0, 1.0)) + (t * Color(0.5, 0.7, 1.0))
        }
    }
}

fn generate_random_scene() -> HitList {
    let mut world = HitList::new();

    let ground_material = Rc::new(Lambertian::from_color(Color(0.5, 0.5, 0.5)));

    world.add(Box::new(Sphere::new(
        Point3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand();

            let center = Point3(
                (a as f64) + (0.9 * rand()),
                0.2,
                (b as f64) + (0.9 * rand()),
            );

            let object: Box<dyn Hit>;

            if (center - Point3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Color::rand() * Color::rand();

                    sphere_material = Rc::new(Lambertian::from_color(albedo));

                    let center2 = center + Vec3(0.0, rand_between(0.0, 0.5), 0.0);
                    object = Box::new(MovingSphere::new(
                        center,
                        center2,
                        Interval::new(0.0, 1.0),
                        0.2,
                        sphere_material,
                    ));
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Color::rand_between(0.5, 1.0);
                    let fuzz = rand_between(0.0, 0.5);

                    sphere_material = Rc::new(Metal::new(albedo, fuzz));

                    object = Box::new(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // Glass
                    sphere_material = Rc::new(Dielectric::new(1.5));

                    object = Box::new(Sphere::new(center, 0.2, sphere_material));
                }

                world.add(object);
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    let material2 = Rc::new(Lambertian::from_color(Color(0.4, 0.4, 0.1)));
    let material3 = Rc::new(Metal::new(Color(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::new(Point3(0.0, 1.0, 0.0), 1.0, material1)));

    world.add(Box::new(Sphere::new(
        Point3(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    world.add(Box::new(Sphere::new(Point3(4.0, 1.0, 0.0), 1.0, material3)));
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

    let options = Options {
        vertical_fov: 20.0,
        aspect_ratio,
        aperture,
        focus_distance: distance_to_focus,
    };

    let camera = Camera::new(position, look_at, up, options, Interval::new(0.0, 1.0));

    println!("P3\n{} {}\n255", image_width, image_height);

    for line in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", line);

        for column in 0..image_width {
            let mut pixel_color = Color(0.0, 0.0, 0.0);

            for _sample in 0..samples_per_pixel {
                let u = (column as f64 + rand()) / (image_width - 1) as f64;
                let v = (line as f64 + rand()) / (image_height - 1) as f64;

                let ray = camera.ray(u, v);

                pixel_color += ray_color(ray, &world, max_depth);
            }

            write_color(&mut stdout(), pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}
