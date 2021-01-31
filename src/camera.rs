use crate::ray::Ray;
use crate::util::cross;
use crate::vec3::{Point3, Vec3};

pub const APSECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vfov: f64) -> Self {
        let h = (vfov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = APSECT_RATIO * viewport_height;

        let vup = Vec3::new(0.0, 1.0, 0.0);
        let w = (lookfrom - lookat).normalize();
        let u = cross(&vup, &w).normalize();
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
