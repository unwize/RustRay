use std::rc::Rc;
use glam::Vec3;
use crate::material::Material;
use crate::primitives::Intersectable;
use crate::ray::Ray;

/// A struct containing actionable data regarding the intersection of a ray and a primitive
pub struct Hit {
    pub entry: f32,  // Length at which the ray intersected the primitive closest to ray origin
    pub exit: f32,  // Length at which the ray intersected the primitive farthest from ray origin
    pub ray: Ray,  // The ray that generated the hit
    pub normal_entry: Option<Vec3>,  // Normal at the point of entry
    pub normal_exit: Option<Vec3>,  // Normal at the point of exit
    pub material: Material   // Material at point of entry
}

impl Hit {
    pub fn new(entry: f32, exit: f32, ray: Ray, normal_entry: Option<Vec3>, normal_exit: Option<Vec3>, material: Material) -> Self {
        Self {
            entry,
            exit,
            ray,
            normal_entry,
            normal_exit,
            material
        }
    }

    pub fn entry_point(&self) -> Vec3 {
        (self.ray.direction.normalize() * self.entry) + self.ray.origin
    }

    pub fn exit_point(&self) -> Vec3 {
        (self.ray.direction.normalize() * self.exit) + self.ray.origin
    }
}