use std::ops::RangeInclusive;
use std::rc::Rc;

use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub enum Normal {
    Front(Vec3),
    Back(Vec3),
}

pub struct HitRecord {
    pub point: Point3,
    pub t: f64,
    pub normal: Normal,
}

impl HitRecord {
    pub fn normal(&self) -> Vec3 {
        match self.normal {
            Normal::Front(v) => v,
            Normal::Back(v) => v,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
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