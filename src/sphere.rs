use crate::*;
use std::sync::Arc;

pub struct Sphere {
    pub center: (Point3, Point3),
    pub time: (f64, f64),
    pub radius: f64,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center: (center, center),
            time: (0.0, 1.0),
            radius,
            material,
        }
    }

    pub fn movement(mut self, center: Point3, time: (f64, f64)) -> Self {
        self.center.1 = center;
        self.time = time;
        self
    }

    fn center(&self, time: f64) -> Point3 {
        self.center.0
            + ((time - self.time.0) / (self.time.1 - self.time.0)) * (self.center.1 - self.center.0)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(r.time);
        let oc = r.origin - center;
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
                let outward_normal = (hit_record.position - center) / self.radius;
                hit_record.set_face_normal(r, outward_normal);
                return Some(hit_record);
            }
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let radius_vec = Vec3(self.radius.abs(), self.radius.abs(), self.radius.abs());
        let center_0 = self.center(t0);
        let center_1 = self.center(t1);
        let bound_box_0 = AABB::new(center_0 - radius_vec, center_0 + radius_vec);
        let bound_box_1 = AABB::new(center_1 - radius_vec, center_1 + radius_vec);
        Some(bound_box_0.surrounding_box(&bound_box_1))
    }
}
