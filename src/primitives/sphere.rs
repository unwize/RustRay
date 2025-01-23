use crate::primitives::{Colored, Intersectable};
use crate::ray::Ray;
use crate::render::Scene;
use glam::f32::Vec3;
use image::Rgb;

pub struct Sphere {
    origin: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f32) -> Self {
        Self { origin, radius }
    }
}

impl Colored for Sphere {
    fn get_point_color(&self, camera_ray: &Ray, intersecting_point: &Vec3, scene: &Scene) -> Rgb<u8> {
        Rgb([255, 255, 255])
    }
}

impl Intersectable for Sphere {
    // See: https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
    // d = sqrt(L^2 -tca^2). If d < 0, return false.
    // L = vector from eye origin to sphere origin
    // tca = line from origin that forms a right-angle triangle with L

    /// Computes the intersection between a normalized ray and self. If ray is not normalized,
    /// unexpected behavior may occur.
    fn intersect(&self, ray: &Ray) -> Option<Vec<Vec3>> {

        //solve for tc
        let L: Vec3 = self.origin - ray.origin;
        let tc: f32 = L.dot(ray.direction);

        if tc < 0.0 {
            //println!("Bailed on a TC < 0");
            return None;
        }
        let d2 = tc * tc - L.length_squared();

        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            //println!("Bailed on a short d2");
            return None;
        }

        //solve for t1c
        let t1c = radius2 - d2;

        //solve for intersection points
        let t1 = tc - t1c;
        let t2 = tc + t1c;

        let p1 = ray.direction * t1;
        let p2 = ray.direction * t2;

        println!("Found intersections: {p1}, {p2}");
        Some(vec![p1, p2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_origin() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 3.0), 1.0);
        let intersections = sphere.intersect(&ray);
        assert!(intersections.is_some(), "No intersections found");
        let values = intersections.unwrap();
        assert_eq!(values.len(), 2, "Incorrect number of intersections detected");
        assert!(values.iter().any(|p| {p.eq(&Vec3::new(0.0, 0.0, 2.0))}), "Front-side intersection missing from values");
        assert!(values.iter().any(|p| {p.eq(&Vec3::new(0.0, 0.0, 4.0))}), "Back-side intersection missing from values");
    }

    #[test]
    fn test_intersect_cross() {
        let origin = Vec3::new(0.0, 0.0, 12.0);
        let radius: f32 = 6.0;
        let sphere = Sphere::new(origin.clone(), radius);

        let mut offset = -radius;

        loop {
            if offset > radius {
                break;
            }

            println!("Offset: {offset:?}");
            let projection = origin + Vec3::new(offset, -offset, 0.0);
            let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), projection.normalize());
            println!("Testing ray: {ray}");
            assert!(sphere.intersect(&ray).is_some());
            offset += 1.0
        }

    }
}