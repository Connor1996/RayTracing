use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }
}
