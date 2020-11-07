use crate::core::color::Color;
use crate::core::geometry::Ray;
use crate::scene::BasicHitRecord;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, in_ray: Ray, hit: BasicHitRecord) -> Option<ScatterRecord>;
}

pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
