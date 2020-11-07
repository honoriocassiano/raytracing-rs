use crate::core::color::Color;
use crate::core::geometry::Ray3;
use crate::scene::BasicHitRecord;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub ray: Ray3,
}

pub trait Material {
    fn scatter(&self, in_ray: Ray3, hit: BasicHitRecord) -> Option<ScatterRecord>;
}
