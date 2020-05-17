use std::sync::Arc;

use crate::*;

pub struct Cube {
    min: Vec3,
    max: Vec3,
    sides: HitList,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Arc<dyn Material + Send + Sync>) -> Arc<Self> {
        let mut sides = HitList::new();
        sides.add(AxisRectangle::new(
            "Z",
            (min.x(), max.x()),
            (min.y(), max.y()),
            (max.z(), max.z()),
            material.clone(),
        ));
        sides.add(FlipFace::new(AxisRectangle::new(
            "Z",
            (min.x(), max.x()),
            (min.y(), max.y()),
            (min.z(), min.z()),
            material.clone(),
        )));

        sides.add(AxisRectangle::new(
            "Y",
            (min.x(), max.x()),
            (max.y(), max.y()),
            (min.z(), max.z()),
            material.clone(),
        ));
        sides.add(FlipFace::new(AxisRectangle::new(
            "Y",
            (min.x(), max.x()),
            (min.y(), min.y()),
            (min.z(), max.z()),
            material.clone(),
        )));

        sides.add(AxisRectangle::new(
            "X",
            (max.x(), max.x()),
            (min.y(), max.y()),
            (min.z(), max.z()),
            material.clone(),
        ));
        sides.add(FlipFace::new(AxisRectangle::new(
            "X",
            (min.x(), min.x()),
            (min.y(), max.y()),
            (min.z(), max.z()),
            material.clone(),
        )));

        Arc::new(Self { min, max, sides })
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
