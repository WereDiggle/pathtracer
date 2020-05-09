use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use crate::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x().fract();
        let v = p.y().fract();
        let w = p.z().fract();

        //
        let i = (4.0 * p.x()) as usize & 255;
        let j = (4.0 * p.y()) as usize & 255;
        let k = (4.0 * p.z()) as usize & 255;

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut ranfloat: Vec<f64> = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranfloat.push(rng.gen());
        }

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    fn generate_perm(rng: &mut ThreadRng) -> Vec<usize> {
        let mut p: Vec<usize> = (0..POINT_COUNT).collect();

        for i in (1..POINT_COUNT).rev() {
            let target: usize = rng.gen_range(0, i);
            p.swap(i, target);
        }

        p
    }
}
