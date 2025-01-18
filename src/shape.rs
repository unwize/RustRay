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

struct Sphere {
    origin: IVec3,
    radius: i32,
}

impl Intersectable for Sphere {
    // See: https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
    // d = sqrt(L^2 -tca^2). If d < 0, return false.
    // L = vector from eye origin to sphere origin
    // tca = line from origin that forms a right-angle triangle with L
    fn intersect(&self, mut ray: &Ray) -> Option<Vec<IVec3>> {
        ray.normalize();

        //solve for tc
        let L: IVec3 = self.origin - ray.origin.unwrap_or(IVec3::new(0, 0, 0));
        let tc = L.dot(&ray.direction);


        if tc < 0 {
            return None;
        }

        let d2 = (tc*tc) - (L.dot(&L));

        let radius2 = self.radius * self.radius;
        if  d2 > radius2 {
            return None;
        }

        //solve for t1c
        let t1c = ((radius2 - d2) as f32).sqrt();

        //solve for intersection points
        let t1 = tc - t1c as i32;
        let t2 = tc + t1c as i32;

        Some(vec![ray.direction * t1, ray.direction * t2])
    }
}

