use crate::*;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3;
}

pub struct SolidColor(pub Color3);

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color3 {
        self.0
    }
}
