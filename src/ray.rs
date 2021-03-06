use crate::hittable::Hittable;
use crate::vec3::{Color, Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    org: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(org: Point3, dir: Vec3) -> Self {
        Self { org, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.org
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.org + t * self.dir
    }

    pub fn color<H: Hittable>(&self, world: &H, depth: u16) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        world
            .hit(self, 0.001, std::f64::INFINITY)
            .map(|i| {
                i.mat
                    .scatter(self, i)
                    .map(|(attenuation, scattered)| attenuation * scattered.color(world, depth - 1))
                    .unwrap_or(Color::default())
            })
            .unwrap_or({
                let unit_dir = self.dir.unit();
                let t = 0.5 * (unit_dir.y() + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::vec3::{Point3, Vec3};

    #[test]
    fn test_at() {
        let ray = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 2.0, 1.0));
        let actual = ray.at(2.0);
        let expected = Point3::new(1.0, 2.0, 3.0) + 2.0 * Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(actual, expected);
    }
}
