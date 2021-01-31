use std::ops::RangeInclusive;

use crate::vec3::{Point3};
use crate::ray::Ray;

pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        AABB {
            minimum: a,
            maximum: b,
        }
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Point3::new(
            box0.minimum.x().min(box1.minimum.x()),
            box0.minimum.y().min(box1.minimum.y()),
            box0.minimum.z().min(box1.minimum.z()),
        );
        let big = Point3::new(
            box0.maximum.x().min(box1.maximum.x()),
            box0.maximum.y().min(box1.maximum.y()),
            box0.maximum.z().min(box1.maximum.z()),
        );
        AABB::new(small, big)
    }

    pub fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> bool {
        let mut t_min = *t_range.start();
        let mut t_max = *t_range.end();
        for i in 0..3 {
            let invD = 1.0 / ray.direction()[i];

            let mut t0 = (self.minimum[i] - ray.origin()[i]) * invD; 
            let mut t1 = (self.maximum[i]- ray.origin()[i]) * invD;
            if invD < 0.0 {
               std::mem::swap(&mut t0, &mut t1); 
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_min >= t_max {
                return false;
            }
        }
        true
    }
}