use crate::core::geometry::{Point3, Ray3, Vec3, Vector};
use crate::core::math::geometry::degrees_to_radians;
use crate::core::math::rand::rand_in_unit_disk;

#[allow(dead_code)]
pub struct Camera {
    position: Point3,
    lower_left_corner: Point3,

    horizontal: Vec3,
    vertical: Vec3,

    u: Vec3,
    v: Vec3,
    w: Vec3,

    lens_radius: f64,
}

impl Camera {
    pub fn new(
        position: Point3,
        look_at: Point3,
        up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta: f64 = degrees_to_radians(vertical_fov);
        let h: f64 = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (position - look_at).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u);

        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;

        // let lower_left_corner = position - horizontal / 2.0 - vertical / 2.0 - w;
        let lower_left_corner =
            position - (horizontal / 2.0) - (vertical / 2.0) - (focus_distance * w);

        let lens_radius = aperture / 2.0;

        Self {
            position,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray3 {
        let rd: Vec3 = self.lens_radius * rand_in_unit_disk();
        let offset: Vec3 = (self.u * rd.x()) + (self.v * rd.y());

        let origin = self.position + offset;
        let direction = self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
            - self.position
            - offset;

        Ray3::new(origin, direction)

        // Ray {
        // 	origin: self.position,
        // 	direction: self.lower_left_corner + (s * self.horizontal) + (t * self.vertical) - self.position,
        // }
    }
}
