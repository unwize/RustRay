use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// A minimalist implementation of 3D math vector (not programming vector).
impl IVec3 {

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn to_array(&self) -> [i32; 3] {
        [self.x, self.y, self.z]
    }

    fn from_array(array: [i32; 3]) -> Self {
        IVec3 { x: array[0], y: array[1], z: array[2] }
    }

    /// Returns a new vector where each element is equal to the passed value
    pub(crate) fn splat(value: i32) -> Self {
        IVec3 { x: value, y: value, z: value }
    }

    fn from_slice(slice: &[i32]) -> Self {
        IVec3 { x: slice[0], y: slice[1], z: slice[2] }
    }

    /// Returns an int containing the dot product of self and the passed vector
    pub fn dot(&self, rhs: &Self) -> i32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Returns a new vector that is a dot product of self and the passed vector
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    /// Returns a new vector containing the absolute value of self
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn length_squared(&self) -> i32 {
        self.dot(self)
    }

    pub fn distance_squared(&self, rhs: &Self) -> i32 {
        (self - rhs).length_squared()
    }

    pub fn sqrt(&self) -> Self {
        Self {
            x : (self.x as f64).sqrt() as i32,
            y: (self.y as f64).sqrt() as i32,
            z: (self.z as f64).sqrt() as i32,
        }
    }
}

impl Div<IVec3> for IVec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z),
        }
    }
}

impl Div<&IVec3> for IVec3 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: &IVec3) -> IVec3 {
        self.div(*rhs)
    }
}

impl Div<&IVec3> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: &IVec3) -> IVec3 {
        (*self).div(*rhs)
    }
}

impl Div<IVec3> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: IVec3) -> IVec3 {
        (*self).div(rhs)
    }
}

impl DivAssign<IVec3> for IVec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
    }
}

impl DivAssign<&Self> for IVec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs)
    }
}

impl Div<i32> for IVec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i32) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}

impl Div<&i32> for IVec3 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: &i32) -> IVec3 {
        self.div(*rhs)
    }
}

impl Div<&i32> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: &i32) -> IVec3 {
        (*self).div(*rhs)
    }
}

impl Div<i32> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: i32) -> IVec3 {
        (*self).div(rhs)
    }
}

impl DivAssign<i32> for IVec3 {
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.z.div_assign(rhs);
    }
}

impl DivAssign<&i32> for IVec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &i32) {
        self.div_assign(*rhs)
    }
}

impl Div<IVec3> for i32 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: IVec3) -> IVec3 {
        IVec3 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
        }
    }
}

impl Div<&IVec3> for i32 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: &IVec3) -> IVec3 {
        self.div(*rhs)
    }
}

impl Div<&IVec3> for &i32 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: &IVec3) -> IVec3 {
        (*self).div(*rhs)
    }
}

impl Div<IVec3> for &i32 {
    type Output = IVec3;
    #[inline]
    fn div(self, rhs: IVec3) -> IVec3 {
        (*self).div(rhs)
    }
}

impl Mul<IVec3> for IVec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
        }
    }
}

impl Mul<&IVec3> for IVec3 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: &IVec3) -> IVec3 {
        self.mul(*rhs)
    }
}

impl Mul<&IVec3> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: &IVec3) -> IVec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<IVec3> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: IVec3) -> IVec3 {
        (*self).mul(rhs)
    }
}

impl MulAssign<IVec3> for IVec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
    }
}

impl MulAssign<&Self> for IVec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs)
    }
}

impl Mul<i32> for IVec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}

impl Mul<&i32> for IVec3 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: &i32) -> IVec3 {
        self.mul(*rhs)
    }
}

impl Mul<&i32> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: &i32) -> IVec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<i32> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: i32) -> IVec3 {
        (*self).mul(rhs)
    }
}

impl MulAssign<i32> for IVec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
        self.z.mul_assign(rhs);
    }
}

impl MulAssign<&i32> for IVec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &i32) {
        self.mul_assign(*rhs)
    }
}

impl Mul<IVec3> for i32 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: IVec3) -> IVec3 {
        IVec3 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
        }
    }
}

impl Mul<&IVec3> for i32 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: &IVec3) -> IVec3 {
        self.mul(*rhs)
    }
}

impl Mul<&IVec3> for &i32 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: &IVec3) -> IVec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<IVec3> for &i32 {
    type Output = IVec3;
    #[inline]
    fn mul(self, rhs: IVec3) -> IVec3 {
        (*self).mul(rhs)
    }
}

impl Add<IVec3> for IVec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl Add<&IVec3> for IVec3 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: &IVec3) -> IVec3 {
        self.add(*rhs)
    }
}

impl Add<&IVec3> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: &IVec3) -> IVec3 {
        (*self).add(*rhs)
    }
}

impl Add<IVec3> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: IVec3) -> IVec3 {
        (*self).add(rhs)
    }
}

impl AddAssign<IVec3> for IVec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}

impl AddAssign<&Self> for IVec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs)
    }
}

impl Add<i32> for IVec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: i32) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
        }
    }
}

impl Add<&i32> for IVec3 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: &i32) -> IVec3 {
        self.add(*rhs)
    }
}

impl Add<&i32> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: &i32) -> IVec3 {
        (*self).add(*rhs)
    }
}

impl Add<i32> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: i32) -> IVec3 {
        (*self).add(rhs)
    }
}

impl AddAssign<i32> for IVec3 {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
        self.z.add_assign(rhs);
    }
}

impl AddAssign<&i32> for IVec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &i32) {
        self.add_assign(*rhs)
    }
}

impl Add<IVec3> for i32 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: IVec3) -> IVec3 {
        IVec3 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
        }
    }
}

impl Add<&IVec3> for i32 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: &IVec3) -> IVec3 {
        self.add(*rhs)
    }
}

impl Add<&IVec3> for &i32 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: &IVec3) -> IVec3 {
        (*self).add(*rhs)
    }
}

impl Add<IVec3> for &i32 {
    type Output = IVec3;
    #[inline]
    fn add(self, rhs: IVec3) -> IVec3 {
        (*self).add(rhs)
    }
}

impl Sub<IVec3> for IVec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}

impl Sub<&IVec3> for IVec3 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: &IVec3) -> IVec3 {
        self.sub(*rhs)
    }
}

impl Sub<&IVec3> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: &IVec3) -> IVec3 {
        (*self).sub(*rhs)
    }
}

impl Sub<IVec3> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: IVec3) -> IVec3 {
        (*self).sub(rhs)
    }
}

impl SubAssign<IVec3> for IVec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: IVec3) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
    }
}

impl SubAssign<&Self> for IVec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs)
    }
}

impl Sub<i32> for IVec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: i32) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
        }
    }
}

impl Sub<&i32> for IVec3 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: &i32) -> IVec3 {
        self.sub(*rhs)
    }
}

impl Sub<&i32> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: &i32) -> IVec3 {
        (*self).sub(*rhs)
    }
}

impl Sub<i32> for &IVec3 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: i32) -> IVec3 {
        (*self).sub(rhs)
    }
}

impl SubAssign<i32> for IVec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
    }
}

impl SubAssign<&i32> for IVec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &i32) {
        self.sub_assign(*rhs)
    }
}

impl Sub<IVec3> for i32 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: IVec3) -> IVec3 {
        IVec3 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
        }
    }
}

impl Sub<&IVec3> for i32 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: &IVec3) -> IVec3 {
        self.sub(*rhs)
    }
}

impl Sub<&IVec3> for &i32 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: &IVec3) -> IVec3 {
        (*self).sub(*rhs)
    }
}

impl Sub<IVec3> for &i32 {
    type Output = IVec3;
    #[inline]
    fn sub(self, rhs: IVec3) -> IVec3 {
        (*self).sub(rhs)
    }
}