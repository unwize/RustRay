use crate::color::Color;
use crate::structures::IVec3;

pub struct Light {
    origin: IVec3,
    color: Color,
    intensity: f64,
}