pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// A minimalist implementation of 3D math vector (not programming vector).
impl IVec3 {
    fn to_array(&self) -> [i32; 3] {
        [self.x, self.y, self.z]
    }

    fn from_array(array: [i32; 3]) -> Self {
        IVec3 { x: array[0], y: array[1], z: array[2] }
    }

    /// Returns a new vector where each element is equal to the passed value
    fn splat(value: i32) -> Self {
        IVec3 { x: value, y: value, z: value }
    }

    fn from_slice(slice: &[i32]) -> Self {
        IVec3 { x: slice[0], y: slice[1], z: slice[2] }
    }

    /// Returns an int containing the dot product of self and the passed vector
    pub fn dot(self, rhs: Self) -> i32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Returns a new vector that is a dot product of self and the passed vector
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    /// Returns a new vector containing the absolute value of self
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}