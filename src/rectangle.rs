use crate::*;

pub struct ZRectangle {
    material: Arc<dyn Material + Send + Sync>,
    x: (f64, f64),
    y: (f64, f64),
    z: f64,
}

impl ZRectangle {
    pub fn new(
        x: (f64, f64),
        y: (f64, f64),
        z: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        assert!(x.0 < x.1);
        assert!(y.0 < y.1);
        Self { x, y, z, material }
    }
}

impl Hittable for ZRectangle {
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3(self.x.0, self.y.0, self.z - 0.0001),
            Vec3(self.x.1, self.y.1, self.z - 0.0001),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let distance = (self.z - r.origin.z()) / r.direction.z();
        if distance < t_min || distance > t_max {
            return None;
        }

        let x = r.origin.x() + distance * r.direction.x();
        if x < self.x.0 || x > self.x.1 {
            return None;
        }
        let y = r.origin.y() + distance * r.direction.y();
        if y < self.y.0 || y > self.y.1 {
            return None;
        }

        let z = r.origin.z() + distance * r.direction.y();
        let mut hit = HitRecord::from_material(self.material.clone());
        hit.u = (x - self.x.1) / (self.x.1 - self.x.0);
        hit.v = (y - self.y.1) / (self.y.1 - self.y.0);
        hit.position = Vec3(x, y, z);
        hit.distance = distance;
        hit.set_face_normal(r, Vec3(0.0, 0.0, 1.0));
        Some(hit)
    }
}
