use crate::*;
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
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
            let distance = (-half_b - root) / a;
            let distance = if distance < t_max && distance > t_min {
                distance
            } else {
                (-half_b + root) / a
            };

            if distance < t_max && distance > t_min {
                let mut hit_record = HitRecord::from_material(self.material.clone());
                hit_record.position = r.at(distance);
                hit_record.distance = distance;
                let outward_normal = (hit_record.position - self.center) / self.radius;
                hit_record.set_face_normal(r, outward_normal);
                return Some(hit_record);
            }
        }
        None
    }
}
