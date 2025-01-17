use crate::ray::Ray;
use crate::color::Color;
use crate::structures::IVec3;
use crate::lighting::Light;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Vec<IVec3>>;
}

pub trait Colored {
    fn get_color(light_sources: &Vec<Light>) -> Color;
}

struct Sphere {
    origin: IVec3,
    radius: i32,
}

impl Intersectable for Sphere {
    // See: https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
    // d = sqrt(L^2 -tca^2). If d < 0, return false.
    // L = vector from eye origin to sphere origin
    // tca = line from origin that forms a right-angle triangle with L
    fn intersect(&self, ray: &Ray) -> Option<Vec<IVec3>> {
        let L: IVec3 = self.origin.clone() - ray.squish();
        let tc: i32 = L.dot(&ray.direction);

        if tc < 0 {
            return None;
        }

        let tca = ray.direction.clone() * tc;

        let d = (tca * tca) - (L * L);
        let d = d.sqrt();

        if d.length_squared() > self.radius * 2 {
            return None;
        }

        return Some(vec![])
    }
}

