mod vec;
mod color;
mod ray;

pub use vec::Vec3;
pub use vec::dot;
pub use vec::cross;

pub use vec::Vec3 as Point3;

pub use color::Color;
pub use color::write_color;

pub use ray::Ray;