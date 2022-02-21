use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::hittable::Intersection;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, i: Intersection) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, i: Intersection) -> Option<(Color, Ray)> {
        let scatter_direction = match i.normal + Vec3::random_unit_vector() {
            dir if dir.near_zero() => i.normal,
            dir => dir,
        };

        Some((self.albedo, Ray::new(i.p, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, i: Intersection) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().unit().reflect(i.normal);
        let out = (
            self.albedo,
            Ray::new(i.p, reflected + self.fuzz * Vec3::random_in_unit_sphere()),
        );

        if out.1.direction().dot(i.normal) > 0.0 {
            Some(out)
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, i: Intersection) -> Option<(Color, Ray)> {
        let refraction_ratio = if i.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = r_in.direction().unit();
        let cos_theta = (-unit_direction).dot(i.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let is_reflective = SmallRng::from_rng(rand::thread_rng())
            .map(|mut rng| Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0))
            .unwrap_or(false);

        let direction = if refraction_ratio * sin_theta > 1.0 || is_reflective {
            unit_direction.reflect(i.normal)
        } else {
            unit_direction.refract(i.normal, refraction_ratio)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(i.p, direction)))
    }
}
