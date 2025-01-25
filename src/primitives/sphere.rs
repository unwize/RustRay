use crate::material::Material;
use crate::primitives::{Intersectable, Materialed};
use crate::ray::Ray;
use crate::structures::Hit;
use glam::f32::Vec3;

pub struct Sphere {
    origin: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f32, material: Material) -> Self {
        Self { origin, radius, material }
    }
}

impl Materialed for Sphere {

    fn get_material(&self) -> &Material {
        &self.material
    }
}

impl Intersectable for Sphere {
    // See: https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
    // d = sqrt(L^2 -tca^2). If d < 0, return false.
    // L = vector from eye origin to sphere origin
    // tca = line from origin that forms a right-angle triangle with L

    /// Computes the intersection between a normalized ray and self. If ray is not normalized,
    /// unexpected behavior may occur.
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let q = self.origin - ray.origin;
        let v_dot_q = ray.direction.dot(q);
        let d_sqrd = q.dot(q) - self.radius*self.radius;
        let discriminant = v_dot_q * v_dot_q - d_sqrd;

        if discriminant >= 0.0 {
            let root = discriminant.sqrt();
            let t0 = v_dot_q - root;
            let t1 = v_dot_q + root;

            let entry: f32;
            let exit: f32;

            if t0 < t1 {
                entry = t0;
                exit = t1;
            } else {
                entry = t1;
                exit = t0;
            }

            let mut normal_entry: Option<Vec3>  = None;
            let mut normal_exit: Option<Vec3> = None;

            if entry > 0.0 {
                // hit.normal = self.normal(ray.pos(hit.entry));
                normal_entry = Some(self.normal(ray.direction * entry))
            }
            if exit > 0.0 {
                // hit.normal2 = self.normal(ray.pos(hit.exit));
                normal_exit = Some(self.normal(ray.direction * exit))
            }

            let hit = Hit::new(entry, exit, ray.clone(), normal_entry, normal_exit, self.material.clone());

            let p0 = hit.entry_point();
            let p1 = hit.exit_point();
            println!("Hit @{p0}, {p1}");

            return Some(hit)

        }

        None
    }

    fn normal(&self, point: Vec3) -> Vec3 {
        point - self.origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;

    fn get_material() -> Material {
        Material::new(Rgb([0, 0, 1]), None, None, None)
    }

    #[test]
    fn test_intersect_origin() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 3.0), 1.0, get_material());
        let intersections = sphere.intersect(&ray);
        assert!(intersections.is_some(), "No intersections found");
        let hit = intersections.unwrap();
        assert!(hit.entry > 0.0, "Entry point missed, improperly");
        assert!(hit.exit > 0.0, "Exit point missed, improperly");
        assert_eq!(hit.entry * hit.ray.direction, Vec3::new(0.0, 0.0, 2.0));
        assert_eq!(hit.exit * hit.ray.direction, Vec3::new(0.0, 0.0, 4.0));
    }

    #[test]
    fn test_intersect_cross() {
        let origin = Vec3::new(0.0, 0.0, 12.0);
        let radius: f32 = 6.0;
        let sphere = Sphere::new(origin, radius, get_material());

        let mut offset = 0.0;

        loop {
            if offset > radius {
                break;
            }

            println!("Offset: {offset:?}");
            let projection = origin + Vec3::new(offset, -offset, 12.0);
            let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), projection.normalize());
            println!("Testing ray: {ray}");
            assert!(sphere.intersect(&ray).is_some());
            offset += 1.0
        }
    }

    #[test]
    fn test_intersect_miss_behind() {
        let origin = Vec3::new(0.0, 0.0, -12.0);
        let radius: f32 = 6.0;
        let sphere = Sphere::new(origin, radius, get_material());

        let mut offset = 0.0;

        loop {
            if offset > radius {
                break;
            }

            println!("Offset: {offset:?}");
            let projection = origin + Vec3::new(offset, -offset, 12.0);
            let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), projection.normalize());
            println!("Testing ray: {ray}");
            assert!(sphere.intersect(&ray).is_none());
            offset += 1.0
        }
    }
}