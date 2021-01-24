use crate::hit::{HitRecord, Normal, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot,  Point3};
use std::ops::RangeInclusive;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center, 
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        let origin = ray.origin();
        let direction = ray.direction();
    
        let oc = origin - self.center;
        let a = dot(&direction, &direction);
        let b = 2.0 * dot(&oc, &direction);
        let c = dot(&oc, &oc) - self.radius* self.radius;
    
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            // Find the nearest root that lies in the acceptable Range
            let mut root = (-b - discriminant.sqrt()) / (2.0 * a);
            if !t_range.contains(&root) {
                root = (-b + discriminant.sqrt()) / (2.0 * a);
                if !t_range.contains(&root) {
                    return None;
                }
            }
            let hit = ray.at(root);
            let normal =  (hit - self.center) / self.radius;
            Some(HitRecord{
                p: hit,
                t: root,
                normal: if dot(&direction, &normal) < 0.0 {
                    Normal::Front(normal)
                } else {
                    Normal::Back(-normal)
                },
            })
        }
    }
}