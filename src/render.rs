use glam::f32::Vec2;
use glam::Vec3;
use crate::lighting::Light;
use crate::primitives::Intersectable;
use crate::ray::Ray;
use crate::primitives::plane::FinitePlane;

struct Renderer {
    ambient_light: Light,
    point_lights: Vec<Light>,
    primitives: Vec<Box<dyn Intersectable>>,
    camera_pos: Vec3,
    camera_direction: Vec3,
    screen_plane: FinitePlane,
    screen_resolution: [u32; 2],
    fov: u32
}

impl Renderer {

    // Screen-space transformations: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays.html
    fn get_pixel_ray(&self, pixel: [u32; 2]) -> Vec3 {
        let aspec_ratio = self.screen_resolution[0] as f32 / self.screen_resolution[1] as f32;
        let x = pixel[0] as f32 + 0.5;
        let y = pixel[1] as f32 + 0.5;

        let pixel_ndc = Vec2::new(x/ self.screen_resolution[0] as f32, y / self.screen_resolution[1] as f32);
        let pixel_screen = Vec2::new(1.0 - 2.0 * pixel_ndc.x, 1.0 - 2.0 * pixel_ndc.y);

        let fov_coef = f32::tan(self.fov as f32 / 2.0);
        let pixel_camera: Vec2 = Vec2::new(
            (2.0 * pixel_screen.x - 1.0) * aspec_ratio * fov_coef,
            1.0 - 2.0 * pixel_screen.y *  fov_coef
        );

        (Vec3::new(pixel_camera.x, pixel_camera.y, -1.0) - self.camera_pos + self.camera_direction.normalize()).normalize()

    }
    pub fn trace(&self, pixel: [u32; 2]) {

    }
}