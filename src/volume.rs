use rand::random;

use crate::*;

pub struct ConstantMedium {
    // Assume a convex shape.
    boundary: Arc<dyn Hittable + Send + Sync>,
    phase_function: Arc<dyn Material + Send + Sync>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable + Send + Sync>,
        texture: Arc<dyn Texture + Send + Sync>,
        density: f64,
    ) -> Arc<Self> {
        Arc::new(Self {
            boundary,
            phase_function: Isotropic::from_texture(texture),
            neg_inv_density: -1.0 / density,
        })
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec1 = self
            .boundary
            .hit(r, std::f64::NEG_INFINITY, std::f64::INFINITY)?;
        let mut rec2 = self
            .boundary
            .hit(r, rec1.distance + 0.0001, std::f64::INFINITY)?;

        rec1.distance = rec1.distance.max(t_min);
        rec2.distance = rec2.distance.min(t_max);

        if rec1.distance >= rec2.distance {
            return None;
        }

        rec1.distance = rec1.distance.max(0.0);

        let distance_inside_boundary = rec2.distance - rec1.distance;
        let hit_distance = self.neg_inv_density * random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let distance = rec1.distance + hit_distance;
        let position = r.at(distance);
        Some(HitRecord {
            distance,
            position,
            normal: Vec3(1.0, 0.0, 0.0),
            front_face: true,
            material: self.phase_function.clone(),
            u: 0.0,
            v: 0.0,
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
