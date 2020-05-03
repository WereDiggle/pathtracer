use crate::hit::*;
use crate::ray::*;
use crate::vec3::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let position = r.at(temp);
                return Some(HitRecord {
                    t: temp,
                    position,
                    normal: (position - self.center) / self.radius,
                });
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let position = r.at(temp);
                return Some(HitRecord {
                    t: temp,
                    position,
                    normal: (position - self.center) / self.radius,
                });
            }
        }
        None
    }
}
