use crate::core::geometry::{Point3, Ray, Ray3, Vec3};

#[derive(Copy, Clone)]
pub struct TimeRay3 {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

#[allow(dead_code)]
impl TimeRay3 {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> TimeRay3 {
        TimeRay3 {
            origin,
            direction,
            time,
        }
    }

    pub fn from_ray(ray: Ray3) -> TimeRay3 {
        TimeRay3::new(ray.origin(), ray.direction(), 0.0)
    }

    pub fn to_ray(self) -> Ray3 {
        Ray3::new(self.origin(), self.direction())
    }

    pub fn time(self) -> f64 {
        self.time
    }
}

impl Ray for TimeRay3 {
    type Point = Point3;
    type Vector = Vec3;
    type Scalar = f64;

    fn at(&self, t: f64) -> Self::Point {
        self.origin + (self.direction * t)
    }

    fn origin(self) -> Self::Point {
        self.origin
    }

    fn direction(self) -> Self::Vector {
        self.direction
    }
}
