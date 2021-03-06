use crate::core::geometry::{Point2, Point3, Ray, Vector};
use crate::materials::Material;
use crate::scene::{Hit, MaterialHitRecord};

use crate::core::math::constants::PI;
use crate::core::time::{Interval, TimeRay3};
use crate::scene::object::AABB;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn get_uv(point: Point3) -> Point2 {
        let theta = (-point.y()).acos();
        let phi = (-point.z()).atan2(point.x()) + PI;

        let x = phi / (2.0 * PI);
        let y = theta / PI;

        Point2(x, y)
    }

    pub const fn center(&self) -> Point3 {
        self.center
    }

    pub const fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: TimeRay3, t_min: f64, t_max: f64) -> Option<MaterialHitRecord> {
        let oc = ray.origin() - self.center;

        let a: f64 = ray.direction().sq_length();
        let half_b: f64 = oc.dot(ray.direction());
        let c: f64 = oc.sq_length() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root: f64 = discriminant.sqrt();

            let check_hit = |val: f64| -> Option<MaterialHitRecord> {
                if (t_min < val) && (val < t_max) {
                    let t = val;
                    let point = ray.at(val);

                    let outward_normal = (point - self.center) / self.radius;
                    let text_coord = Self::get_uv(outward_normal);

                    return Some(MaterialHitRecord::new(
                        point,
                        t,
                        // FIXME Use TimeRay3 here
                        ray.to_ray(),
                        text_coord,
                        outward_normal,
                        self.material.clone(),
                    ));
                }

                None
            };

            return match check_hit((-half_b - root) / a) {
                Some(val) => Some(val),
                None => check_hit((-half_b + root) / a),
            };
        }

        None
    }

    fn bounding_box(&self, _interval: Interval) -> Option<AABB> {
        let radius_vec = Point3(self.radius, self.radius, self.radius);

        Some(AABB::new(
            self.center - radius_vec,
            self.center + radius_vec,
        ))
    }
}
