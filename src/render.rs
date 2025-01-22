use std::rc::Rc;
use crate::lighting::Light;
use crate::primitives::Intersectable;
use crate::ray::Ray;
use glam::Vec3;
use image::{ImageBuffer, Rgb};

/// Main data structure that contains all the necessary data to represent a scene
pub struct Renderer {
    pub scene: Scene,
    pub camera: Camera,
    pub(crate) image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl Renderer {
    pub fn new(scene: Scene, camera: Camera) -> Renderer {
        let w = camera.image_dimensions[0];
        let l = camera.image_dimensions[1];
        Renderer {
            scene,
            camera,
            image_buffer: ImageBuffer::new(w, l)
        }
    }
}

pub struct Scene {
    pub ambient_light: Light,
    pub point_lights: Vec<Light>,
    pub primitives: Vec<Rc<Box<dyn Intersectable>>>,
}

/// Simple struct that contains camera-specific data
pub struct Camera {
    pub focal_length: f32,
    pub origin_ray: Ray,
    pub fov: u32,
    pub image_dimensions: [u32; 2],

    _aspect_ratio: Option<f32>,
    _viewport_dimensions: Option<[f32; 2]>
}

impl Camera {
    pub(crate) fn new(focal_length: f32, origin_ray: Ray, fov: u32, image_dimensions: [u32; 2]) -> Self {
        Self {
            focal_length,
            origin_ray,
            fov,
            image_dimensions,
            _aspect_ratio: None,
            _viewport_dimensions : None,
        }
    }

    /// Compute the aspect ratio of the image from the user-defined image height and width.
    /// Cache the result as image dimensions should not change.
    fn aspect_ratio(&mut self) -> f32 {
        if self._aspect_ratio == None {
            self._aspect_ratio = Some(self.image_dimensions[0] as f32 / self.image_dimensions[1] as f32)
        }

        self._aspect_ratio.unwrap()
    }

    /// Fetch viewport dimensions. These are calculated and cached if they haven't been already.
    fn viewport_dimensions(&mut self) -> [f32; 2] {
        if self._viewport_dimensions == None {
            self._viewport_dimensions = Some([2.0, 2.0 * self.aspect_ratio()])
        }

        self._viewport_dimensions.unwrap()
    }
}

impl Renderer {


    /// Main rendering function call. Generates camera rays and stores resultant color values
    pub fn render(&mut self) {
        // Get vectors describing the magnitude of the plane of the viewport
        let viewport_u = Vec3::new(self.camera.viewport_dimensions()[0], 0.0, 0.0);   // width
        let viewport_v = Vec3::new(0.0, -self.camera.viewport_dimensions()[1], 0.0);  // height

        // Calculate vectors representing the distance between the center of each pixel (delta vectors)
        let pixel_delta_u = viewport_u / self.camera.image_dimensions[0] as f32;
        let pixel_delta_v = viewport_v / self.camera.image_dimensions[1] as f32;

        // Calculate the location of the upper left pixel (0,0)
        let viewport_root = self.camera.origin_ray.origin
            - Vec3::new(0.0, 0.0, self.camera.focal_length)
            - viewport_u/2.0 - viewport_v/2.0;  // The corner of the viewport plane
        let root_pixel_pos = viewport_root + 0.5 * (pixel_delta_u + pixel_delta_v);  // Offset the corner of the viewport plane using the delta vectors to get the center of the pixel

        // For each pixel
        for y in 0..self.camera.image_dimensions[1] {
            let v : Vec<Rgb<u8>> = vec![];
            for x in 0..self.camera.image_dimensions[0] {

                // Compute the center of the pixel and then create a ray with which to cast for collision
                let pixel_center = root_pixel_pos + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
                let camera_ray_direction = pixel_center - self.camera.origin_ray.origin;
                let camera_ray: Ray = Ray::new(self.camera.origin_ray.origin, camera_ray_direction);

                &self.image_buffer.put_pixel(x, y, self.cast_ray(camera_ray));
            }
        }
    }

    /// Compute if any primitives in the scene intersect with the ray and store the color of the
    /// primitive in the image data.
    fn cast_ray(&self, camera_ray: Ray) -> Rgb<u8> {
        let mut closest_intersection: f32 = f32::MAX;
        let mut closest_primitive: Option<Rc<Box<dyn Intersectable>>> = None;
        let mut closest_primitive_point: Vec3 = Vec3::MAX;
        for primitive in &self.scene.primitives {
            let intersections = primitive.intersect(&camera_ray);

            // If there is at least one point of intersection
            if intersections.is_some() {

                // For each point of intersection
                for point in intersections.unwrap() {

                    // Compute distance from the point to the camera. Discard all points that are
                    // farther from the camera than the closest point
                    let point_distance = self.camera.origin_ray.origin.distance(point);
                    if point_distance < closest_intersection {
                        closest_intersection = point_distance;
                        closest_primitive = Some(primitive.clone());
                        closest_primitive_point = point;
                    }
                }
            }
        }

        if closest_primitive.is_some() {
            return closest_primitive.unwrap().get_point_color(&camera_ray, &closest_primitive_point, &self.scene)
        }

        Rgb([0,0,0])
    }

}