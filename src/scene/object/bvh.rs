use crate::core::math::rand::rand_between;
use crate::core::time::{Interval, TimeRay3};
use crate::scene::object::AABB;
use crate::scene::{Hit, MaterialHitRecord};
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BVH {
    left: Rc<dyn Hit>,
    right: Rc<dyn Hit>,
    bounding_box: AABB,
}

#[allow(dead_code)]
impl BVH {
    pub fn from_objects(source_objects: &[Rc<(dyn Hit)>], interval: Interval) -> Self {
        let left: Rc<dyn Hit>;
        let right: Rc<dyn Hit>;

        let axis = rand_between(0, 3);

        let object_span = source_objects.len();

        match object_span {
            1 => {
                left = source_objects[0].clone();
                right = source_objects[0].clone();
            }
            2 => {
                let compare =
                    Hit::box_compare(source_objects[0].as_ref(), source_objects[1].as_ref(), axis);

                if compare == Ordering::Less {
                    left = source_objects[0].clone();
                    right = source_objects[1].clone();
                } else {
                    left = source_objects[1].clone();
                    right = source_objects[0].clone();
                }
            }
            _ => {
                let mut vec = Vec::from(source_objects);

                vec.sort_by(|a, b| Hit::box_compare(a.as_ref(), b.as_ref(), axis));

                let mid = vec.len() / 2;

                left = Rc::new(BVH::from_objects(&vec[..mid], interval));
                right = Rc::new(BVH::from_objects(&vec[mid..], interval));
            }
        }

        let box_left: AABB;
        let box_right: AABB;

        if let (Some(l), Some(r)) = (left.bounding_box(interval), right.bounding_box(interval)) {
            box_left = l;
            box_right = r;
        } else {
            // TODO Print error here
            panic!("No bounding box in BVH constructor");
        }

        Self {
            left,
            right,
            bounding_box: AABB::surrounding_box(&box_left, &box_right),
        }
    }

    pub fn left(&self) -> &Rc<dyn Hit> {
        &self.left
    }

    pub fn right(&self) -> &Rc<dyn Hit> {
        &self.right
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

impl Hit for BVH {
    fn hit(&self, ray: TimeRay3, t_min: f64, t_max: f64) -> Option<MaterialHitRecord> {
        if !self.bounding_box.hit(&ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);

        match hit_left {
            None => self.right.hit(ray, t_min, t_max),
            Some(hit) => self.right.hit(ray, t_min, hit.t()),
        }
    }

    fn bounding_box(&self, _interval: Interval) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::color::Color;
    use crate::core::geometry::{Point3, Vec3};
    use crate::core::time::TimeRay3;
    use crate::materials::Lambertian;
    use crate::scene::object::sphere::Sphere;
    use crate::scene::{Hit, HitList};
    use std::rc::Rc;

    fn make_static_scene() -> HitList {
        let material = Rc::new(Lambertian::from_color(Color(0.6, 0.6, 0.6)));

        let sphere1 = Sphere::new(Point3(0.0, 1.0, 0.0), 1.0, material.clone());
        let sphere2 = Sphere::new(Point3(0.0, -1.0, 0.0), 1.0, material);

        let mut list = HitList::new();
        list.add(Box::new(sphere1));
        list.add(Box::new(sphere2));

        list
    }

    #[test]
    fn must_hit_something() {
        let scene = make_static_scene();

        let ray = TimeRay3::new(Point3(-2.0, 0.1, 0.0), Vec3(5.0, 0.0, 0.0), 0.0);

        let hit = scene.hit(ray, 0.0, 1.0);

        assert!(hit.is_some());
    }
}
