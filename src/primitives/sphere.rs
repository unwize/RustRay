use crate::primitives::Intersectable;
use crate::ray::Ray;
use crate::structures::IVec3;

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
        let l: IVec3 = self.origin - ray.origin;
        let tc = l.dot(&ray.direction);


        if tc < 0 {
            return None;
        }

        let d2 = (tc*tc) - (l.dot(&l));

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