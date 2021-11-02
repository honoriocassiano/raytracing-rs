use crate::core::color::Color;
use crate::core::math::rand::rand_unit_vector;
use crate::scene::BasicHitRecord;

use super::material::{Material, ScatterRecord};
use crate::core::time::TimeRay3;
use crate::textures::{SolidColor, Texture};
use std::sync::Arc;

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

#[allow(dead_code)]
impl Lambertian {
    pub fn new(color: Arc<dyn Texture>) -> Self {
        Self { albedo: color }
    }

    pub fn from_color(color: Color) -> Self {
        let albedo = Arc::new(SolidColor::new(color));

        Self { albedo }
    }

    pub fn albedo(&self) -> Arc<dyn Texture> {
        self.albedo.clone()
    }
}

impl Material for Lambertian {
    fn scatter(&self, in_ray: TimeRay3, hit: BasicHitRecord) -> Option<ScatterRecord> {
        let scatter_direction = hit.normal() + rand_unit_vector();

        let scatter_record = ScatterRecord {
            ray: TimeRay3::new(hit.point(), scatter_direction, in_ray.time()),
            attenuation: self.albedo.value(hit.texture_coordinate(), hit.point()),
        };

        Some(scatter_record)
    }
}
