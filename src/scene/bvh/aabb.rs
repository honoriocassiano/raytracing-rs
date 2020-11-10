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
}
