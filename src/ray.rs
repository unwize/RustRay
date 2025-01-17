use crate::structures::IVec3;

// A pair of i32 arrays containing an optional origin and a required direction vector.
// When origin is None, assume that the origin is the eye position
pub struct Ray {
    pub origin: Option<IVec3>,
    pub direction: IVec3,
}

impl Ray {
    fn magnitude(&self) -> i32 {
        ((
            self.direction.x * self.direction.x
                + self.direction.y * self.direction.x
                + self.direction.z * self.direction.z
        ) as f64).sqrt() as i32
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.direction = IVec3 {
            x : self.direction.x/magnitude,
            y : self.direction.y/magnitude,
            z : self.direction.z/magnitude
        };
    }

    pub fn squish(&self) -> IVec3 {
        self.origin.unwrap_or(IVec3::new(0,0,0)) + self.direction
    }
}