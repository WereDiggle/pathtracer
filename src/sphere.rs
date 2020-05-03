use crate::*;

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
                let mut hit_record = HitRecord::at_position_and_distance(r.at(temp), temp);
                let outward_normal = (hit_record.position - self.center) / self.radius;
                hit_record.set_face_normal(r, outward_normal);
                return Some(hit_record);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let mut hit_record = HitRecord::at_position_and_distance(r.at(temp), temp);
                let outward_normal = (hit_record.position - self.center) / self.radius;
                hit_record.set_face_normal(r, outward_normal);
                return Some(hit_record);
            }
        }
        None
    }
}
