use crate::core::color::Color;
use crate::core::geometry::{Vec2, Vec3};
use crate::textures::Texture;

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _coord: Vec2, _point: Vec3) -> Color {
        self.color
    }
}
