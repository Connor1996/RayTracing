use std::ops::Range;

use crate::vec3::{Point3, Vec3};

use rand::Rng;

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

pub fn random_f64() -> f64 {
    random_f64_range(0.0..1.0)
}

pub fn random_f64_range(range: Range<f64>) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

pub fn random_in_unit_sphere() -> Point3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if p.length_squared() > 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_disk() -> Point3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() > 1.0 {
            continue;
        }
        return p;
    }
}

// True Lambertian Reflection is achieved by picking random points on the surface of the unit sphere, offset along the surface normal.
// Picking random points on the unit sphere can be achieved by picking random points in the unit sphere, and then normalizing those.
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - *normal * dot(&v, &normal) * 2.0
}

pub fn refract(v: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = 1.0_f64.min(dot(&-*v, normal));

    let r_out_perp = (*v + *normal * cos_theta) * etai_over_etat;
    let r_out_parallel = *normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_parallel + r_out_perp
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
