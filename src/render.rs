use glam::f32::Vec2;
use crate::lighting::Light;
use crate::primitives::Intersectable;
use crate::ray::Ray;
use crate::primitives::plane::FinitePlane;

struct Renderer {
    ambient_light: Light,
    point_lights: Vec<Light>,
    primitives: Vec<Box<dyn Intersectable>>,
    eye_ray: Ray,
    screen_plane: FinitePlane,
    screen_resolution: [u32; 2],
}

impl Renderer {

    // Screen-space transformations: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays.html
    fn camera_to_world(&self, pixel: [u32; 2]) -> Vec2 {
        let aspec_ratio = self.screen_resolution[0] as f32 / self.screen_resolution[1] as f32;
        let x = pixel[0] as f32 + 0.5;
        let y = pixel[1] as f32 + 0.5;

        let pixel_ndc = Vec2::new(x/ self.screen_resolution[0] as f32, y / self.screen_resolution[1] as f32);
        let pixel_screen = Vec2::new(1.0 - 2.0 * pixel_ndc.x, 1.0 - 2.0 * pixel_ndc.y);
    }
    pub fn trace(&self, pixel: [u32; 2]) {

    }
}