mod vec;
mod color;
mod ray;
pub mod camera;
mod vector;
mod optic;

pub use vec::Vec3;
pub use vector::Vector;
pub use vec::Vec3 as Point3;

pub use color::Color;
pub use color::write_color;

pub use ray::Ray;

pub use camera::Camera;

pub use optic::Reflect;
pub use optic::Refract;
