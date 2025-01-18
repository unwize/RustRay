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

// Bounded plane intersection math: https://stackoverflow.com/questions/56316509/ray-bounded-plane-intersection
pub struct FinitePlane {
    position: IVec3,
    u: IVec3,
    v: IVec3,
}

impl Intersectable for FinitePlane {
    fn intersect(&self, ray: &Ray) -> Option<Vec<IVec3>> {
        let normal = self.u.cross(&self.v);
        let u_d_u = self.u.dot(&self.u);
        let u_d_v = self.u.dot(&self.v);
        let v_d_v = self.v.dot(&self.v);
        let det = (u_d_u * v_d_v) - (u_d_v * u_d_v);

        let t = normal.dot(&(&self.position - &ray.origin)) / normal.dot(&ray.direction);
        let int_point = ray.origin + t * ray.direction;
        let rhs = int_point - self.position;

        //u_dot_rhs = dot(u, rhs);
        let u_d_rhs = self.u.dot(&rhs);

        //v_dot_rhs = dot(v, rhs);
        let v_d_rhs = self.v.dot(&rhs);

        //w1 = (v_dot_v * u_dot_rhs - u_dot_v * v_dot_rhs) / det;
        let w1 = (v_d_v * u_d_rhs - u_d_v * v_d_rhs) / det;

        //w2 = (- u_dot_v * u_dot_rhs + u_dot_u * v_dot_rhs) / det;
        let w2 = (- u_d_v * u_d_rhs + u_d_u * v_d_rhs)/det;

        if (0 <= w1 && w1 <= 1 && 0 <= w2 && w2 <= 1 ){
            Some(vec![int_point])
        }
        else{
            None
        }
    }
}

