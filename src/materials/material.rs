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
