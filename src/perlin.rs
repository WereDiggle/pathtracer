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
        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;

        let u = (1.0 + p.x().fract()).fract();
        let v = (1.0 + p.y().fract()).fract();
        let w = (1.0 + p.z().fract()).fract();
        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let rand_value = self.ranfloat[self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize]];
                    c[di][dj][dk] = rand_value;
                }
            }
        }

        trilinear_interp(&c, u, v, w)
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

fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                    * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                    * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }

    accum
}
