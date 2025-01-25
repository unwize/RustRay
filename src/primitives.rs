pub mod plane;
pub mod sphere;

use crate::material::Material;
use crate::ray::Ray;
use crate::structures::Hit;
use glam::f32::Vec3;

/// A required trait for rendering in a scene. Provides functionality to determine if a ray has hit the associated primitive and at what points.
pub trait Intersectable : Materialed {
    /// Takes a reference to a ray and calculates points of intersection with self. If there are no
    /// points, None is returned instead.
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    fn normal(&self, point: Vec3) -> Vec3;
}

/// A required trait for rendering in a scene. Provides functionality to determine the color of the associated primitive.
pub trait Materialed {
    fn get_material(&self) -> &Material;
}