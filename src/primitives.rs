pub mod plane;
pub mod sphere;

use crate::ray::Ray;
use crate::render::Scene;
use glam::f32::Vec3;
use image::Rgb;

/// A required trait for rendering in a scene. Provides functionality to determine if a ray has hit the associated primitive and at what points.
pub trait Intersectable : Colored {
    /// Takes a reference to a ray and calculates points of intersection with self. If there are no
    /// points, None is returned instead.
    fn intersect(&self, ray: &Ray) -> Option<Vec<Vec3>>;
}

/// A required trait for rendering in a scene. Provides functionality to determine the color of the associated primitive.
pub trait Colored {
    fn get_point_color(&self, camera_ray: &Ray, intersecting_point: &Vec3, scene: &Scene) -> Rgb<u8>;
}