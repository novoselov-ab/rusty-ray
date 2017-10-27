use std::fmt;
use vec3::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    #[inline]
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray { origin: origin, dir: dir }
    }

    pub fn point(self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}
