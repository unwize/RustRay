use crate::ray::Ray;
use crate::color::Color;
use crate::structures::IVec3;
use crate::lighting::Light;

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> bool;
}

pub trait Colored {
    fn get_color(light_sources: &Vec<Light>) -> Color;
}

struct Sphere {
    origin: IVec3,
    radius: IVec3,
}

impl Intersectable for Sphere {
   //d = sqrt(L^2 -tca^2). If d < 0, return false.
   // L = vector from eye origin to sphere origin
    fn intersect(&self, ray: Ray) -> bool {
        let L = self.origin - ray.origin + ray.direction;
    }
}

