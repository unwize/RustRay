use crate::color::Color;
use glam::f32::Vec3;
pub struct Light {
    origin: Vec3,
    color: Color,
    intensity: f64,
}