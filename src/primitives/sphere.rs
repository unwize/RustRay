use crate::material::Material;
use crate::primitives::{Hit, Intersectable, Materialed};
use crate::ray::Ray;
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

        let mut hits: Option<Vec<Vec3>> = None;
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

            let normal: Option<Vec3>  = None

            if entry > 0.0 {
                // hit.normal = self.normal(ray.pos(hit.entry));
                normal = Some(self.normal(ray.direction * entry))
            }
            if exit > 0.0 {
                // hit.normal2 = self.normal(ray.pos(hit.exit));
                normal = Some(self.normal(ray.direction * exit))
            }

            let hit = Hit::new(entry, exit, ray.clone(), Vec3::default(), &self.material);

        }
    }


        return hit

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