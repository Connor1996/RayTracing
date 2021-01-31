use crate::ray::Ray;
use crate::util::{cross, random_f64_range, random_in_unit_disk};
use crate::vec3::{Point3, Vec3};

pub const APSECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,

    // u,v,w are three orthonormal unit vectors to describe our camera’s orientation.
    u: Vec3,
    v: Vec3,
    w: Vec3,

    len_radius: f64,

    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vfov: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let h = (vfov.to_radians() / 2.0).tan() * focus_dist; // the viewing is at z=-focus_dist
        let viewport_height = 2.0 * h;
        let viewport_width = APSECT_RATIO * viewport_height;

        let vup = Vec3::new(0.0, 1.0, 0.0);
        let w = (lookfrom - lookat).normalize();
        let u = cross(&vup, &w).normalize();
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            len_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.len_radius;
        let offset = rd * self.u * self.v;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t
                - (self.origin + offset),
            random_f64_range(self.time0..self.time1),
        )
    }
}
