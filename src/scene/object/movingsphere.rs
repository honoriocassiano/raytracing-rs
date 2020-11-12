use crate::core::geometry::{Point3, Ray, Vec3, Vector};
use crate::materials::Material;
use crate::scene::{Hit, MaterialHitRecord};

use crate::core::time::{Interval, TimeRay3, Timestamp};
use crate::scene::object::AABB;
use std::rc::Rc;

pub struct MovingSphere {
    start_center: Point3,
    end_center: Point3,
    time_interval: Interval,
    radius: f64,
    material: Rc<dyn Material>,
}

#[allow(dead_code)]
impl MovingSphere {
    pub fn new(
        start_center: Point3,
        end_center: Point3,
        time_interval: Interval,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            start_center,
            end_center,
            time_interval,
            radius,
            material,
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self, timestamp: Timestamp) -> Point3 {
        let i = &self.time_interval;

        self.start_center
            + ((timestamp - i.start()) / (i.end() - i.start()))
                * (self.end_center - self.start_center)
    }
}

impl Hit for MovingSphere {
    fn hit(&self, ray: TimeRay3, t_min: f64, t_max: f64) -> Option<MaterialHitRecord> {
        let oc = ray.origin() - self.center(ray.time());

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

                    let outward_normal = (point - self.center(ray.time())) / self.radius;

                    return Some(MaterialHitRecord::new(
                        point,
                        t,
                        // FIXME Use TimeRay3 here
                        ray.to_ray(),
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

    fn bounding_box(&self, interval: Interval) -> Option<AABB> {
        let radius_vec = Vec3(self.radius, self.radius, self.radius);

        let box0 = AABB::new(
            self.center(interval.start()) - radius_vec,
            self.center(interval.start()) + radius_vec,
        );

        let box1 = AABB::new(
            self.center(interval.end()) - radius_vec,
            self.center(interval.end()) + radius_vec,
        );

        Some(box0.surrounding_box(&box1))
    }
}
