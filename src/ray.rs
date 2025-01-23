use glam::f32::Vec3;

// A pair of i32 arrays containing an optional origin and a required direction vector.
// When origin is None, assume that the origin is the eye position
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn magnitude(&self) -> f32 {
        (
            self.direction.x * self.direction.x
                + self.direction.y * self.direction.x
                + self.direction.z * self.direction.z
        ).sqrt()
    }

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