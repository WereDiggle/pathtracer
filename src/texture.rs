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

pub struct CheckerTexture {
    size: f64,
    odd: Arc<dyn Texture + Send + Sync>,
    even: Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new(
        size: f64,
        odd: Arc<dyn Texture + Send + Sync>,
        even: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self { size, odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3 {
        let sines =
            (p.x() * self.size).sin() * (p.y() * self.size).sin() * (p.z() * self.size).sin();
        if sines.is_sign_negative() {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    perlin: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3 {
        Vec3(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.perlin.turb(p, 7)).sin())
    }
}
