use std::ops::{Add, Sub, Neg, Mul, Div};
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0., 0., 0.)
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn scale(self, s: f32) -> Self {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn normalize(self) -> Self {
        let dot = self.dot(self);
        if dot.is_normal() {
            self / dot.sqrt()
        } else {
            self
        }
    }

    pub fn square_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.square_length().sqrt()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: f32) -> Vec3 {
        self.scale(s)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, s: f32) -> Vec3 {
        self.scale(1. / s)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_op_test() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);
        assert!((v1 + v2) == Vec3::new(5., 7., 9.));
        assert!((v1 - v2) == Vec3::new(-3., -3., -3.));
        assert!(Vec3::zero() == Vec3::new(0., 0., 0.));
        assert!(-v1 == Vec3::new(-1., -2., -3.));
        assert!(v1.dot(v2) == 32.);
        assert!(v1.scale(2.) == Vec3::new(2., 4., 6.));
        assert!(v1 * 2. == v1.scale(2.));
        assert!(v1 / 2. == v1.scale(1. / 2.));
        assert!(v1.normalize() == v1 / v1.length());
        assert!(v1.square_length() == (1. + 2. * 2. + 3. * 3.));
        assert!(Vec3::new(1., 0., 0.).cross(Vec3::new(0., 1., 0.)) == Vec3::new(0., 0., 1.));
    }
}
