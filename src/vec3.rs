use std::ops;

use image::*;
use rand::random;
use rand::thread_rng;
use rand::Rng;

use crate::util::Clamp;

/// Simple 3 data point structure
///
/// Useful for points, vectors, normals, and colors
#[derive(Copy, Debug, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;
pub type Color3 = Vec3;

impl Vec3 {
    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn infinity() -> Self {
        Self(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY)
    }

    pub fn neg_infinity() -> Self {
        Self(
            std::f64::NEG_INFINITY,
            std::f64::NEG_INFINITY,
            std::f64::NEG_INFINITY,
        )
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

    pub fn min(&self, other: &Self) -> Self {
        Vec3(
            self.0.min(other.0),
            self.1.min(other.1),
            self.2.min(other.2),
        )
    }

    pub fn max(&self, other: &Self) -> Self {
        Vec3(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vec3) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn reflect(&self, normal: Vec3) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = normal.dot(-*self);
        let r_out_parallel = etai_over_etat * (*self + cos_theta * normal);
        let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * normal;
        r_out_parallel + r_out_perp
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn random() -> Self {
        Vec3(random::<f64>(), random::<f64>(), random::<f64>())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        Vec3(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        Self::random_unit_vector() * random::<f64>()
    }

    pub fn random_unit_vector() -> Self {
        let mut rng = thread_rng();
        let a = rng.gen_range(0.0, std::f64::consts::PI * 2.0);
        let z = rng.gen_range(-1.0, 1.0);
        let r = (1.0f64 - z * z).sqrt();
        Vec3(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = thread_rng();
        let angle = rng.gen_range(0.0, std::f64::consts::PI * 2.0);
        let length: f64 = rng.gen();
        Vec3(length * angle.cos(), length * angle.sin(), 0.0)
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if normal.dot(in_unit_sphere) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_cosine_direction() -> Self {
        let r1 = random::<f64>();
        let r2 = random::<f64>();
        let z = (1.0 - r2).sqrt();

        let phi = 2.0 * std::f64::consts::PI * r1;
        let x = r2.sqrt() * phi.cos();
        let y = r2.sqrt() * phi.sin();

        Vec3(x, y, z)
    }
}

impl From<Vec3> for image::Rgb<u8> {
    fn from(color: Vec3) -> Self {
        Rgb([
            (256.0 * color.0.sqrt().clam(0.0, 0.999)) as u8,
            (256.0 * color.1.sqrt().clam(0.0, 0.999)) as u8,
            (256.0 * color.2.sqrt().clam(0.0, 0.999)) as u8,
        ])
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            num => panic!("index out of bounds for Vec3: {}", num),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            num => panic!("index out of bounds for Vec3: {}", num),
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}
