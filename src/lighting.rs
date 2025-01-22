use glam::f32::Vec3;
use image::Rgb;

pub struct Light {
    origin: Vec3,
    color: Rgb<u8>,
    intensity: f64,
}

impl Light {
    pub fn new(origin: Vec3, color: Rgb<u8>, intensity: f64) -> Light {
        Self {
            origin,
            color,
            intensity,
        }
    }
}