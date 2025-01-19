use glam::{vec3, Vec3};
use crate::color::Color;
use crate::lighting::Light;
use crate::ray::Ray;
use crate::render::{Camera, Scene};

pub mod primitives;
pub mod ray;
mod color;
mod lighting;
mod structures;
mod render;



fn main() {
    let ambient = Light::new(Vec3::default(), Color::new(50,50,50), 0.5);
    let point_lights = vec![];

    let scene = Scene {
        ambient_light: ambient,
        point_lights,
        primitives: vec![],
    };

    let camera = Camera::new(
        1.0,
        Ray::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0)),
        0,
        [200, 100]);
}
