use std::fmt::Display;
use glam::f32::Vec3;

// A pair of i32 arrays containing an optional origin and a required direction vector.
// When origin is None, assume that the origin is the eye position
#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {

    pub fn normalize(&mut self) {
        self.direction = self.direction.normalize();
    }

    pub fn squish(&self) -> Vec3 {
        self.origin + self.direction
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.origin, self.direction, self.direction.length())
    }
}