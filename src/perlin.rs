use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use crate::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn turb(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;

        let u = (1.0 + p.x().fract()).fract();
        let v = (1.0 + p.y().fract()).fract();
        let w = (1.0 + p.z().fract()).fract();
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let rand_vec = self.ranvec[self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize]];
                    let weight_vec = Vec3(u - di as f64, v - dj as f64, w - dk as f64);
                    accum += (di as f64 * uu + (1.0 - di as f64) * (1.0 - uu))
                        * (dj as f64 * vv + (1.0 - dj as f64) * (1.0 - vv))
                        * (dk as f64 * ww + (1.0 - dk as f64) * (1.0 - ww))
                        * rand_vec.dot(weight_vec);
                }
            }
        }

        accum
    }

    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut ranvec: Vec<Vec3> = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranvec.push(Vec3::random_range(-1.0, 1.0).unit_vector());
        }

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            ranvec,
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
