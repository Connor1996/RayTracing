use std::ops::{Add, AddAssign, Sub, Neg, Mul, MulAssign, Index, IndexMut, Div, DivAssign};

pub type Color = Vec3;
pub type Point3 = Vec3;

// #[derive(Copy, Clone)]
// pub struct Color(Vec3);

// impl Color {
//     pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
//         Color(Vec3::new(e0, e1, e2)) 
//     }
// }

// impl Deref for Color {
//     type Target = Vec3;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for Color {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }


impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x().floor(), self.y().floor(), self.z().floor())
    }
}

// #[derive(Copy, Clone)]
// pub struct Point3(Vec3);

// impl Point3 {
//     pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
//         Point3(Vec3::new(e0, e1, e2)) 
//     }
// }

// impl Deref for Point3 {
//     type Target = Vec3;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for Point3 {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl From<Vec3> for Point3 {
//     fn from(v: Vec3) -> Self {
//         Point3(v)
//     }
// }


// impl Add<Vec3> for Point3 {
//     type Output = Self;

//     fn add(self, other: Vec3) -> Self::Output {
//         Self(self.0 + other)
//     }
// }

// impl AddAssign<Vec3> for Point3 {
//     fn add_assign(&mut self, other: Vec3) {
//         self.0 += other
//     }
// }

// impl Sub<Vec3> for Point3 {
//     type Output = Self;

//     fn sub(self, other: Vec3) -> Self::Output {
//         Self(self.0 - other)
//     }
// }

// impl Mul<f64> for Point3 {
//     type Output = Self;

//     fn mul(self, other: f64) -> Self::Output {
//         Self(self.0 * other)
//     }
// }

// impl Mul<Vec3> for Point3 {
//     type Output = Self;

//     fn mul(self, other: Vec3) -> Self::Output {
//         Self(self.0 * other)
//     }
// }

// impl MulAssign<Vec3> for Point3 {
//     fn mul_assign(&mut self, other: Vec3) {
//         self.0 *= other
//     }
// }

// impl Div<f64> for Point3 {
//     type Output = Self;

//     fn div(self, other: f64) -> Self::Output {
//         Self(self.0 / other)
//     }
// }

// impl Div<Vec3> for Point3 {
//     type Output = Self;

//     fn div(self, other: Vec3) -> Self::Output {
//         Self(self.0 / other)
//     }
// }

// impl DivAssign<Vec3> for Point3 {
//     fn div_assign(&mut self, other: Vec3) {
//         self.0 /= other
//     }
// }

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
        (*self / self.length() + Vec3::new(1.0, 1.0, 1.0)) * 0.5
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

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}