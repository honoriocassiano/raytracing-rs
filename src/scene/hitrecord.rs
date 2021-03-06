use crate::core::geometry::{Point3, Ray, Ray3, Vec2, Vec3, Vector};

#[derive(Copy, Clone)]
pub struct BasicHitRecord {
    point: Point3,
    normal: Vec3,
    t: f64,
    texture_coord: Vec2,
    front_face: bool,
}

impl BasicHitRecord {
    pub fn new(
        point: Point3,
        t: f64,
        texture_coord: Vec2,
        ray: Ray3,
        outward_normal: Vec3,
    ) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            front_face,
            t,
            texture_coord,
        }
    }

    pub const fn point(&self) -> Point3 {
        self.point
    }

    pub const fn normal(&self) -> Vec3 {
        self.normal
    }

    pub const fn t(&self) -> f64 {
        self.t
    }

    pub const fn texture_coordinate(&self) -> Vec2 {
        self.texture_coord
    }

    pub const fn front_face(&self) -> bool {
        self.front_face
    }
}
