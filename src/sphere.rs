use crate::hittable::{Hittable, Intersection};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    mat: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat: M) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let oc = r.origin() - self.center;
        let (a, half_b, c) = (
            r.direction().length_squared(),
            oc.dot(r.direction()),
            oc.length_squared() - self.radius * self.radius,
        );

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let t = match ((-half_b - sqrtd) / a, (-half_b + sqrtd) / a) {
            (near, _) if t_min <= near && near <= t_max => near,
            (_, far) if t_min <= far && far <= t_max => far,
            _ => return None,
        };

        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let (normal, front_face) = if r.direction().dot(outward_normal) < 0.0 {
            (outward_normal, true)
        } else {
            (-outward_normal, false)
        };

        Some(Intersection::new(p, normal, &self.mat, t, front_face))
    }
}
