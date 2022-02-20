use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

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

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    fn cross(&self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
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
            256.0 * self.0.clamp(0.0, 0.999),
            256.0 * self.1.clamp(0.0, 0.999),
            256.0 * self.2.clamp(0.0, 0.999),
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
