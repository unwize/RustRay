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
        let l: Vec3 = self.origin - ray.origin;
        let tc = l.dot(ray.direction);


        if tc < 0.0 {
            return None;
        }

        let d2 = (tc*tc) - (l.dot(l));

        let radius2 = self.radius * self.radius;
        if  d2 > radius2 {
            return None;
        }

        //solve for t1c
        let t1c = (radius2 - d2).sqrt();

        //solve for intersection points
        let t1 = tc - t1c;
        let t2 = tc + t1c;

        println!("Found intersections: {t1}, {t2}");
        Some(vec![ray.direction * t1, ray.direction * t2])
    }
}