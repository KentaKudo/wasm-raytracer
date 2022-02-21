use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rand::{rngs::SmallRng, Rng, SeedableRng};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(f64, f64, f64);

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl Vec3 {
    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = match SmallRng::from_rng(rand::thread_rng()) {
            Ok(rng) => rng,
            _ => return Self::default(),
        };

        Self(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let mut p = Self::random(-1.0, 1.0);
            p.2 = 0.0;
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn reflect(self, n: Self) -> Self {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = -self.dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_para = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_para
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Into<Vec<u8>> for Color {
    fn into(self) -> Vec<u8> {
        let (r, g, b) = (
            256.0 * self.0.sqrt().clamp(0.0, 0.999),
            256.0 * self.1.sqrt().clamp(0.0, 0.999),
            256.0 * self.2.sqrt().clamp(0.0, 0.999),
        );

        vec![r as u8, g as u8, b as u8, 255]
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_length() {
        let input = Vec3(1.0, 2.0, 3.0);
        let actual = input.length();
        let expected = (input.0 * input.0 + input.1 * input.1 + input.2 * input.2).sqrt();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_default() {
        let actual = Vec3::default();
        let expected = Vec3(0.0, 0.0, 0.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dot() {
        let (a, b) = (Vec3(1.0, 2.0, 3.0), Vec3(1.0, 2.0, 3.0));
        let actual = a.dot(b);
        let expected = a.0 * b.0 + a.1 * b.1 + a.2 * b.2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_cross() {
        let (a, b) = (Vec3(1.0, 2.0, 3.0), Vec3(1.0, 2.0, 3.0));
        let actual = a.cross(b);
        let expected = Vec3(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0,
        );
        assert_eq!(actual, expected)
    }
}
