use std::ops::{Add, Sub, Neg, Mul, Div};
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    fn zero() -> Vec3 {
        Vec3::new(0., 0., 0.)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
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
    }
}