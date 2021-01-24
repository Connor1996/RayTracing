use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::util::{dot, random_in_unit_sphere, random_unit_vector, reflect};
use crate::vec3::Color;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut direction = hit_record.normal() + random_unit_vector();
        if direction.near_zero() {
            direction = hit_record.normal()
        }
        Some((self.albedo, Ray::new(hit_record.point, direction)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let direction = reflect(&ray.direction().normalize(), &hit_record.normal());
        let scattered = Ray::new(
            hit_record.point,
            direction + random_in_unit_sphere() * self.fuzz,
        );
        if dot(&scattered.direction(), &hit_record.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
