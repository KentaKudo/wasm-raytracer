use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    org: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        let v_height = 2.0;
        let v_width = aspect_ratio * v_height;
        let focal_length = 1.0;

        let origin = Point3::default();
        let horizontal = Vec3::new(v_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, v_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            org: Point3::default(),
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.org,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.org,
        )
    }
}
