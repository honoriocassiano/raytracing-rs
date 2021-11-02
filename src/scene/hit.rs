use crate::core::geometry::{Point3, Ray3, Vec2, Vec3};
use crate::materials::Material;

use super::hitrecord::BasicHitRecord;

use crate::core::time::{Interval, TimeRay3};
use crate::scene::object::AABB;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct MaterialHitRecord {
    hit: BasicHitRecord,
    material: Arc<dyn Material>,
}

#[allow(dead_code)]
impl MaterialHitRecord {
    pub fn new(
        point: Point3,
        t: f64,
        ray: Ray3,
        text_coord: Vec2,
        outward_normal: Vec3,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            hit: BasicHitRecord::new(point, t, text_coord, ray, outward_normal),
            material,
        }
    }

    pub fn from_hit(hit: BasicHitRecord, material: Arc<dyn Material>) -> Self {
        Self { hit, material }
    }

    pub const fn hit(&self) -> BasicHitRecord {
        self.hit
    }

    pub const fn point(&self) -> Point3 {
        self.hit.point()
    }

    pub const fn normal(&self) -> Vec3 {
        self.hit.normal()
    }

    pub const fn t(&self) -> f64 {
        self.hit.t()
    }

    pub const fn front_face(&self) -> bool {
        self.hit.front_face()
    }

    pub fn material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }
}

pub trait Hit {
    fn hit(&self, ray: TimeRay3, t_min: f64, t_max: f64) -> Option<MaterialHitRecord>;

    fn bounding_box(&self, interval: Interval) -> Option<AABB>;

    fn box_compare(&self, other: &dyn Hit, axis: usize) -> Ordering {
        let box_a = self.bounding_box(Interval::new(0.0, 0.0));
        let box_b = other.bounding_box(Interval::new(0.0, 0.0));

        if let (Some(a), Some(b)) = (box_a, box_b) {
            if let Some(ord) = a.min()[axis].partial_cmp(&b.min()[axis]) {
                return ord;
            }
        }

        Ordering::Greater
    }
}

// Hit list
pub struct HitList {
    objects: Vec<Arc<(dyn Hit)>>,
}

unsafe impl Send for HitList {}
unsafe impl Sync for HitList {}

impl HitList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with(object: Arc<dyn Hit>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hit>) {
        self.objects.push(object);
    }
}

impl Hit for HitList {
    fn hit(&self, ray: TimeRay3, t_min: f64, t_max: f64) -> Option<MaterialHitRecord> {
        let mut last_hit: Option<MaterialHitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(value) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = value.t();
                last_hit = Some(value);
            }
        }

        last_hit
    }

    fn bounding_box(&self, interval: Interval) -> Option<AABB> {
        let mut bounding_box = self
            .objects
            .first()
            .and_then(|obj| obj.bounding_box(interval));

        let iterator = self
            .objects
            .iter()
            .skip(1)
            .map(|obj| obj.bounding_box(interval));

        for current_bbox in iterator {
            let bbox = bounding_box
                .zip(current_bbox)
                .map(|(aabb, curr_aabb)| aabb.surrounding_box(&curr_aabb));

            bounding_box = bbox;

            if bounding_box.is_none() {
                break;
            }
        }

        bounding_box
    }
}
