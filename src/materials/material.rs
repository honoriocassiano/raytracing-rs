use crate::core::color::Color;
use crate::core::time::TimeRay3;
use crate::scene::BasicHitRecord;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub ray: TimeRay3,
}

pub trait Material {
    fn scatter(&self, in_ray: TimeRay3, hit: BasicHitRecord) -> Option<ScatterRecord>;
}
