pub mod plane;
pub mod sphere;

use std::rc::Rc;
use crate::material::Material;
use crate::ray::Ray;
use glam::f32::Vec3;

/// A required trait for rendering in a scene. Provides functionality to determine if a ray has hit the associated primitive and at what points.
pub trait Intersectable : Materialed {
    /// Takes a reference to a ray and calculates points of intersection with self. If there are no
    /// points, None is returned instead.
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

/// A required trait for rendering in a scene. Provides functionality to determine the color of the associated primitive.
pub trait Materialed {
    fn get_material(&self) -> &Material;
}

/// A struct containing actionable data regarding the intersection of a ray and a primitive
pub struct Hit {
    pub primitive: Rc<Box<dyn Intersectable>>,
    pub entry: f32,
    pub exit: f32,
    pub ray: Ray,
    pub normal: Vec3,
    pub material: Material
}

impl Hit {
    pub fn new(primitive: Rc<Box<dyn Intersectable>>, entry: f32, exit: f32, ray: Ray, normal: Vec3, material: Material) -> Self {
        Self {
            primitive: primitive.clone(),
            entry,
            exit,
            ray,
            normal,
            material
        }
    }
}