use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};

pub struct Intersection<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(p: Point3, normal: Vec3, mat: &'a dyn Material, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

pub struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn random_scene() -> Result<Self, rand::Error> {
        let mut rng = SmallRng::from_rng(rand::thread_rng())?;

        let mut world = Self::new();

        world.add(Box::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(Color::new(0.5, 0.5, 0.5)),
        )));

        for a in -11..11 {
            for b in -11..11 {
                let center = Point3::new(
                    a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                    0.2,
                    b as f64 + 0.9 * rng.gen_range(0.0..1.0),
                );

                if (center - Point3::new(4., 0.2, 0.)).length() <= 0.9 {
                    continue;
                }

                match rng.gen_range(0.0..1.0) {
                    f if 0.0 <= f && f < 0.8 => {
                        // diffuse
                        let albedo = Color::random(0., 1.) * Color::random(0., 1.);
                        world.add(Box::new(Sphere::new(center, 0.2, Lambertian::new(albedo))));
                    }
                    f if 0.8 <= f && f < 0.95 => {
                        // metal
                        let albedo = Color::random(0.5, 1.);
                        let fuzz = rng.gen_range(0.0..0.5);
                        world.add(Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
                    }
                    _ => world.add(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5)))), // glass
                };
            }
        }

        world.add(Box::new(Sphere::new(
            Point3::new(0., 1., 0.),
            1.,
            Dielectric::new(1.5),
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(-4., 1., 0.),
            1.,
            Lambertian::new(Color::new(0.4, 0.2, 0.1)),
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(4., 1., 0.),
            1.,
            Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
        )));

        Ok(world)
    }

    pub fn add(&mut self, h: Box<dyn Hittable>) {
        self.0.push(h)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut result = None;
        let mut t_closest = t_max;
        for obj in &self.0 {
            if let Some(rec) = obj.hit(r, t_min, t_closest) {
                t_closest = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}
