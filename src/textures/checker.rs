use crate::core::color::Color;
use crate::core::geometry::{Vec2, Vec3};
use crate::textures::{SolidColor, Texture};
use std::rc::Rc;

pub struct Checker {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl Checker {
	#[allow(dead_code)]
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn from_color(odd: Color, even: Color) -> Self {
        Self {
            odd: Rc::new(SolidColor::new(odd)),
            even: Rc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, coord: Vec2, point: Vec3) -> Vec3 {
        let sines = (10.0 * point.x()).sin() * (10.0 * point.y()).sin() * (10.0 * point.z()).sin();

        if sines < 0.0 {
            self.odd.value(coord, point)
        } else {
            self.even.value(coord, point)
        }
    }
}
