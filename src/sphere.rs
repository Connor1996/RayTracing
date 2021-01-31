use std::ops::RangeInclusive;
use std::sync::Arc;

use crate::hit::{HitRecord, Hittable, Normal};
use crate::material::Material;
use crate::ray::Ray;
use crate::util::dot;
use crate::vec3::{ Vec3, Point3};
use crate::aabb::AABB;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
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
        let c = dot(&oc, &oc) - self.radius * self.radius;

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
            let normal = (hit - self.center) / self.radius;
            Some(HitRecord {
                point: hit,
                t: root,
                normal: if dot(&direction, &normal) < 0.0 {
                    Normal::Front(normal)
                } else {
                    Normal::Back(-normal)
                },
                material: self.material.clone(),
            })
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        let origin = ray.origin();
        let direction = ray.direction();

        let oc = origin - self.center(ray.time());
        let a = dot(&direction, &direction);
        let b = 2.0 * dot(&oc, &direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;

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
            let normal = (hit - self.center(ray.time())) / self.radius;
            Some(HitRecord {
                point: hit,
                t: root,
                normal: if dot(&direction, &normal) < 0.0 {
                    Normal::Front(normal)
                } else {
                    Normal::Back(-normal)
                },
                material: self.material.clone(),
            })
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        let box0 =
        AABB::new(
            self.center(self.time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(self.time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 =
        AABB::new(
            self.center(self.time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(self.time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(AABB::surrounding_box(box0, box1))
    }
}
