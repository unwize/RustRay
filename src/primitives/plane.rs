use crate::ray::Ray;
use crate::primitives::Intersectable;
use glam::f32::Vec3;

// Bounded plane intersection math: https://stackoverflow.com/questions/56316509/ray-bounded-plane-intersection
pub struct FinitePlane {
    position: Vec3,
    u: Vec3,
    v: Vec3,
}

impl Intersectable for FinitePlane {
    fn intersect(&self, ray: &Ray) -> Option<Vec<Vec3>> {
        let normal = self.u.cross(self.v);
        let u_d_u = self.u.dot(self.u);
        let u_d_v = self.u.dot(self.v);
        let v_d_v = self.v.dot(self.v);
        let det = (u_d_u * v_d_v) - (u_d_v * u_d_v);

        let t = normal.dot(self.position - ray.origin) / normal.dot(ray.direction);
        let int_point = ray.origin + t * ray.direction;
        let rhs = int_point - self.position;

        //u_dot_rhs = dot(u, rhs);
        let u_d_rhs = self.u.dot(rhs);

        //v_dot_rhs = dot(v, rhs);
        let v_d_rhs = self.v.dot(rhs);

        //w1 = (v_dot_v * u_dot_rhs - u_dot_v * v_dot_rhs) / det;
        let w1 = (v_d_v * u_d_rhs - u_d_v * v_d_rhs) / det;

        //w2 = (- u_dot_v * u_dot_rhs + u_dot_u * v_dot_rhs) / det;
        let w2 = (- u_d_v * u_d_rhs + u_d_u * v_d_rhs)/det;

        if (0.0 <= w1 && w1 <= 1.0 && 0.0 <= w2 && w2 <= 1.0 ){
            Some(vec![int_point])
        }
        else{
            None
        }
    }
}