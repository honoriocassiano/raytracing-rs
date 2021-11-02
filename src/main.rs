use std::io::stdout;
use std::sync::Arc;

use rayon::prelude::*;

use scene::camera::Camera;

use crate::core::color::Color;
use crate::core::color::{write_color, write_line};
use crate::core::geometry::{Point3, Ray, Vec3, Vector};
use crate::core::math::rand::rand;
use crate::core::time::{Interval, TimeRay3};
use crate::scene::camera::Options;
use crate::scene::{Hit, HitList};
use crate::scenes::generate_random_scene;

use self::core::math::constants::INFINITY;

mod core;
mod materials;
mod scene;
mod scenes;
mod textures;

fn ray_color(ray: TimeRay3, world: Arc<HitList>, depth: i32) -> Color {
    // Stop recursion at ray bounce limit
    if depth <= 0 {
        return Color(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, INFINITY) {
        Some(material_hit) => {
            let scatter_record = material_hit.material().scatter(ray, material_hit.hit());

            scatter_record.map_or(Color(0.0, 0.0, 0.0), |scr| {
                scr.attenuation * ray_color(scr.ray, world, depth - 1)
            })
        }
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

    let world = Arc::new(generate_random_scene());

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

        let colors: Vec<Color> = (0..image_width)
            .into_par_iter()
            .map(|column| {
                (0..samples_per_pixel)
                    .into_iter()
                    .map(|_| {
                        let u = (column as f64 + rand()) / (image_width - 1) as f64;
                        let v = (line as f64 + rand()) / (image_height - 1) as f64;

                        let ray = camera.ray(u, v);

                        ray_color(ray, world.clone(), max_depth)
                    })
                    .sum()
            })
            .collect();

        write_line(&mut stdout(), colors, samples_per_pixel);
    }

    eprintln!("\nDone.");
}
