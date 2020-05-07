use crate::*;
/// Axis-Aligned Bounding Box
#[derive(Clone)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        assert!(min[0] <= max[0]);
        assert!(min[1] <= max[1]);
        assert!(min[2] <= max[2]);
        Self { min, max }
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        let mut min = Vec3::zero();
        let mut max = Vec3::zero();
        for a in 0..3 {
            min[a] = self.min[a].min(other.min[a]);
            max[a] = self.max[a].max(other.max[a]);
        }
        assert!(min[0] <= max[0]);
        assert!(min[1] <= max[1]);
        assert!(min[2] <= max[2]);
        Self::new(min, max)
    }

    pub fn volume(&self) -> f64 {
        (self.max.x() - self.min.x())
            * (self.max.y() - self.min.y())
            * (self.max.z() - self.min.z())
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t = (
                (self.min[a] - r.origin[a]) * inv_d,
                (self.max[a] - r.origin[a]) * inv_d,
            );
            if inv_d < 0.0 {
                t = (t.1, t.0)
            }
            let t_min = t.0.max(t_min);
            let t_max = t.1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
