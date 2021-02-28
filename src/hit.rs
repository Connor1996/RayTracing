use std::ops::RangeInclusive;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::bvh::BVH;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub enum Normal {
    Front(Vec3),
    Back(Vec3),
}

pub struct HitRecord {
    pub point: Point3,
    pub t: f64,
    pub normal: Normal, // a unit vector
    pub material: Arc<dyn Material>,

    // the normalized surface coordinates
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn normal(&self) -> Vec3 {
        match self.normal {
            Normal::Front(v) => v,
            Normal::Back(v) => v,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn into_bvh(self) -> BVH {
        BVH::new(&self.objects)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut range = t_range.clone();
        for obj in &self.objects {
            if let Some(hit) = obj.hit(ray, &range) {
                range = RangeInclusive::new(*range.start(), hit.t);
                hit_record = Some(hit);
            }
        }
        hit_record
    }

    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box = None;
        for obj in &self.objects {
            if let Some(b) = obj.bounding_box() {
                if output_box.is_none() {
                    output_box = Some(b);
                } else {
                    output_box = Some(AABB::surrounding_box(output_box.unwrap(), b));
                }
            } else {
                return None;
            }
        }
        output_box
    }
}
