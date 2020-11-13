use crate::core::color::Color;
use crate::core::geometry::Point3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color;
}
