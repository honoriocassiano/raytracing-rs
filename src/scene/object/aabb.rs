use crate::core::geometry::Point3;
use crate::core::time::TimeRay3;

#[derive(Copy, Clone)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min, max }
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    // TODO Use Ray3 here?
    #[allow(dead_code)]
    pub fn hit(&self, ray: &TimeRay3, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let temp0 = (self.min[i] - ray.origin()[i]) / ray.direction()[1];
            let temp1 = (self.max[i] - ray.origin()[i]) / ray.direction()[1];

            let t_min = temp0.min(temp1).min(t_min);
            let t_max = temp0.max(temp1).max(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    // TODO Apply DRY principle here
    pub fn surrounding_box(&self, other: &Self) -> Self {
        let small = {
            let p0 = self.min();
            let p1 = other.min();

            let x = p0.x().min(p1.x());
            let y = p0.y().min(p1.y());
            let z = p0.z().min(p1.z());

            Point3(x, y, z)
        };

        let big = {
            let p0 = self.max();
            let p1 = other.max();

            let x = p0.x().max(p1.x());
            let y = p0.y().max(p1.y());
            let z = p0.z().max(p1.z());

            Point3(x, y, z)
        };

        AABB::new(small, big)
    }
}
