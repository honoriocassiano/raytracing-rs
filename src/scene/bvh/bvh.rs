use crate::core::math::rand::rand_between;
use crate::core::time::{Interval, TimeRay3};
use crate::scene::bvh::AABB;
use crate::scene::{Hit, MaterialHitRecord};
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BVH {
    left: Rc<dyn Hit>,
    right: Rc<dyn Hit>,
    bounding_box: AABB,
}

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

        if let (Some(l), Some(r)) = (
            left.bounding_box(time0, time1),
            right.bounding_box(time0, time1),
        ) {
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

        let hit_left = self.right.hit(ray, t_min, t_max);

        match hit_left {
            None => self.right.hit(ray, t_min, t_max),
            Some(hit) => self.right.hit(ray, t_min, hit.t()),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(self.bounding_box)
    }
}
