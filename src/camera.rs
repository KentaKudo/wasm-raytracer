use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    org: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64, // vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let v_height = 2.0 * h;
        let v_width = aspect_ratio * v_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let org = lookfrom;
        let horizontal = focus_dist * v_width * u;
        let vertical = focus_dist * v_height * v;
        let lower_left_corner = org - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            org,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.org + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.org - offset,
        )
    }
}
