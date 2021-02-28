use crate::hit::{HitRecord, Normal};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::util::{
    dot, random_f64, random_in_unit_sphere, random_unit_vector, reflect, reflectance, refract,
};
use crate::vec3::Color;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Self { albedo: texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut direction = hit_record.normal() + random_unit_vector();
        if direction.near_zero() {
            direction = hit_record.normal()
        }
        Some((
            self.albedo
                .value(hit_record.u, hit_record.v, &hit_record.point),
            Ray::new(hit_record.point, direction, ray.time()),
        ))
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
            fuzz: 1.0_f64.min(fuzz),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let direction = reflect(&ray.direction().normalize(), &hit_record.normal());
        let scattered = Ray::new(
            hit_record.point,
            direction + random_in_unit_sphere() * self.fuzz,
            ray.time(),
        );
        if dot(&scattered.direction(), &hit_record.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = match hit_record.normal {
            Normal::Front(_) => 1.0 / self.ir,
            Normal::Back(_) => self.ir,
        };

        let cos_theta = 1.0_f64.min(dot(&-ray.direction().normalize(), &hit_record.normal()));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            reflect(&ray.direction().normalize(), &hit_record.normal())
        } else {
            refract(
                &ray.direction().normalize(),
                &hit_record.normal(),
                refraction_ratio,
            )
        };
        let scattered = Ray::new(hit_record.point, direction, ray.time());
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
