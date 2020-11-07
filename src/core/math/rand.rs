use rand::prelude::*;

use crate::core::geometry::{Vec3, Vector};
use crate::util::PI;

pub fn rand() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0.0, 1.0)
}

pub fn rand_between(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min, max)
}

pub fn rand_unit_vector() -> Vec3 {
    let azimuth = rand_between(0.0, 2.0 * PI);
    let z = rand_between(-1.0, 1.0);
    let radius = (1.0 - z * z).sqrt();

    Vec3(radius * azimuth.cos(), radius * azimuth.sin(), z)
}

#[allow(dead_code)]
pub fn rand_in_hemisphere(normal: Vec3) -> Vec3 {
    let unit_in_shpere = rand_unit_vector();

    let same_hemispherial = unit_in_shpere.dot(normal) > 0.0;

    match same_hemispherial {
        true => unit_in_shpere,
        false => -unit_in_shpere,
    }
}

pub fn rand_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3(rand_between(-1.0, 1.0), rand_between(-1.0, 1.0), 0.0);

        if p.sq_length() < 1.0 {
            break p;
        }
    }
}
