use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Intersection {
    p: Point3,
    pub normal: Vec3,
    pub t: f64,
    front_face: bool,
}

impl Intersection {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut result = None;
        let mut t_closest = t_max;
        for obj in self {
            if let Some(rec) = obj.hit(r, t_min, t_closest) {
                t_closest = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}
