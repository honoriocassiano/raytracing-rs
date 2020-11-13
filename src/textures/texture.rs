use crate::core::color::Color;
use crate::core::geometry::{Point3, Vec2};

pub trait Texture {
    fn value(&self, coord: Vec2, point: Point3) -> Color;
}
