use std::ops::RangeInclusive;
use std::sync::Arc;

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
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
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
}
