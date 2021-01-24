use crate::vec3::{Point3, Vec3};

use rand::Rng;

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
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

// True Lambertian Reflection is achieved by picking random points on the surface of the unit sphere, offset along the surface normal.
// Picking random points on the unit sphere can be achieved by picking random points in the unit sphere, and then normalizing those.
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - *normal * dot(&v, &normal) * 2.0
}
