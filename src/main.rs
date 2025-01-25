use crate::lighting::Light;
use crate::primitives::sphere::Sphere;
use crate::ray::Ray;
use crate::render::{Camera, Renderer, Scene};
use glam::{vec3, Vec3};
use image::Rgb;
use std::fs::File;
use std::rc::Rc;
use crate::material::Material;

pub mod primitives;
pub mod ray;
mod lighting;
mod structures;
mod render;
mod material;

fn main() {
    let ambient = Light::new(Vec3::default(), Rgb([50,50,50]), 0.5);
    let point_lights = vec![];

    let scene = Scene {
        ambient_light: ambient,
        point_lights,
        primitives: vec![
            Rc::new(
                Box::new(
                    Sphere::new(Vec3::new(0.0, 0.0, 2.0), 2.0, Material::new(Rgb([255, 255, 255]), None, None, None))
                )
            )
        ],
    };

    let camera = Camera::new(
        1.0,
        Ray::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0)),
        0,
        [200, 100]);

    let mut renderer = Renderer::new(scene, camera);
    renderer.render();

    File::create("out/image.PNG").expect("TODO: Failed to create base file");
    renderer.image_buffer.save("./out/image.PNG").unwrap()
}
