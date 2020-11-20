use std::io::stdout;

mod core;
mod materials;
mod scene;
mod scenes;
mod textures;

use self::core::math::constants::INFINITY;
use crate::core::color::write_color;
use crate::core::color::Color;
use crate::core::geometry::{Point3, Ray, Vec3, Vector};
use crate::core::math::rand::rand;
use crate::core::time::{Interval, TimeRay3};
use crate::scene::camera::Options;
use crate::scene::{Hit, HitList};
use crate::scenes::generate_random_scene;
use scene::camera::Camera;

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
