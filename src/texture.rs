use std::path::Path;

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

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let im = image::open(path).unwrap();
        let width = im.width() as usize;
        let height = im.height() as usize;
        let mut data = im.to_rgb().into_raw();
        // Gamma decode
        for datum in data.iter_mut() {
            let float_datum = *datum as f64 * COLOR_SCALE;
            *datum = ((float_datum * float_datum) * 255.0) as u8;
        }
        Self {
            data,
            width,
            height,
            bytes_per_scanline: width * 3,
        }
    }
}

const COLOR_SCALE: f64 = 1.0 / 255.0;

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3 {
        if self.data.is_empty() {
            return Vec3(0.0, 1.0, 1.0);
        }

        let u = u.clam(0.0, 1.0);
        let v = 1.0 - v.clam(0.0, 1.0);

        let i = ((u * self.width as f64) as usize).min(self.width - 1);
        let j = ((v * self.height as f64) as usize).min(self.height - 1);
        let index = j * self.bytes_per_scanline + i * 3;

        Vec3(
            COLOR_SCALE * self.data[index] as f64,
            COLOR_SCALE * self.data[index + 1] as f64,
            COLOR_SCALE * self.data[index + 2] as f64,
        )
    }
}
