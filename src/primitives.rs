pub mod plane;
pub mod sphere;

use crate::ray::Ray;
use crate::color::Color;
use crate::structures::IVec3;
use crate::lighting::Light;

/// A required trait for rendering in a scene. Provides functionality to determine if a ray has hit the associated primitive and at what points.
pub trait Intersectable {
    /// Takes a reference to a ray and calculates points of intersection with self. If there are no
    /// points, None is returned instead.
    fn intersect(&self, ray: &Ray) -> Option<Vec<IVec3>>;
}

/// A required trait for rendering in a scene. Provides functionality to determine the color of the associated primitive.
pub trait Colored {
    fn get_color(light_sources: &Vec<Light>) -> Color;
}