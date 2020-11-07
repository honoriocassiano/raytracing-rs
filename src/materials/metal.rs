use crate::core::color::Color;
use crate::core::geometry::{Ray, Ray3, Vector};
use crate::core::optic::Reflect;

use crate::core::math::rand::rand_unit_vector;
use crate::scene::BasicHitRecord;

use super::material::{Material, ScatterRecord};
use crate::core::time::TimeRay3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

#[allow(dead_code)]
impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz: fuzz.min(1.0), // TODO Check if this value can be negative
        }
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }

    pub fn fuzz(&self) -> f64 {
        self.fuzz
    }
}

impl Material for Metal {
    fn scatter(&self, in_ray: TimeRay3, hit: BasicHitRecord) -> Option<ScatterRecord> {
        let reflected = in_ray.direction().normalized().reflect(hit.normal());

        let scatter_record = ScatterRecord {
            ray: Ray3::new(hit.point(), reflected + self.fuzz * rand_unit_vector()),
            attenuation: self.albedo,
        };

        Some(scatter_record)
    }
}
