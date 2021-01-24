use std::ops::{Add, AddAssign, Sub, Neg, Mul, MulAssign, Index, IndexMut, Div, DivAssign};

pub type Color = Vec3;
pub type Point3 = Vec3;

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", (255.0 * self.x().sqrt()).floor(), (255.0 * self.y().sqrt()).floor(), (255.0 * self.z().sqrt()).floor())
    }
}

#[derive(Copy, Clone)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3(e0, e1, e2) 
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    
    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn normalize(&self) -> Self {
        *self / self.length() 
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("out of bound"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("out of bound"),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.x() + other.x(), self.y() + other.y(), self.z() + other.z());
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Self(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}


impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self(self.x() * other.x(), self.y() * other.y(), self.z() * other.z());
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.x() / other.x(), self.y() / other.y(), self.z() / other.z())
    }
}


impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self(self.x() / other, self.y() / other, self.z() / other)
    }
}


impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self(self.x() / other.x(), self.y() / other.y(), self.z() / other.z());
    }
}
