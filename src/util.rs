use crate::core::math::constants::PI;

pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
