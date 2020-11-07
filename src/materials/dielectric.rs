use crate::core::color::Color;
use crate::core::geometry::{Ray, Ray3, Vec3, Vector};
use crate::core::math::optic::schlick;
use crate::core::optic::{Reflect, Refract};

use crate::scene::BasicHitRecord;

use crate::core::math::rand::rand;

use super::material::{Material, ScatterRecord};

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    #[allow(dead_code)]
    pub fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    fn is_reflection(
        &self,
        unit_direction: &Vec3,
        hit: &BasicHitRecord,
        cos_theta: f64,
        sin_theta: f64,
    ) -> Option<Vec3> {
        let eta_in_over_eta_out = {
            if hit.front_face() {
                1.0 / self.refractive_index
            } else {
                self.refractive_index
            }
        };

        if (eta_in_over_eta_out * sin_theta) > 1.0 {
            Some(unit_direction.reflect(hit.normal()))
        } else {
            let reflection_prob = schlick(cos_theta, eta_in_over_eta_out);

            if rand() < reflection_prob {
                Some(unit_direction.reflect(hit.normal()))
            } else {
                None
            }
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, in_ray: Ray3, hit: BasicHitRecord) -> Option<ScatterRecord> {
        let eta_in_over_eta_out = {
            if hit.front_face() {
                1.0 / self.refractive_index
            } else {
                self.refractive_index
            }
        };

        let unit_direction = in_ray.direction().normalized();

        let cos_theta = (-unit_direction).dot(hit.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let scatter_direction: Vec3 = {
            match self.is_reflection(&unit_direction, &hit, cos_theta, sin_theta) {
                Some(reflected) => reflected,
                None => unit_direction.refract(hit.normal(), eta_in_over_eta_out),
            }
        };

        let scatter_record = ScatterRecord {
            ray: Ray3::new(hit.point(), scatter_direction),
            attenuation: Color(1.0, 1.0, 1.0),
        };

        Some(scatter_record)
    }
}
