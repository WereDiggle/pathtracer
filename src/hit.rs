use std::sync::Arc;

use crate::*;

pub struct HitRecord {
    pub material: Arc<dyn Material>,
    pub position: Vec3,
    pub normal: Vec3,
    pub distance: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn from_material(material: Arc<dyn Material>) -> Self {
        Self {
            position: Vec3::zero(),
            normal: Vec3::zero(),
            distance: 0.0,
            front_face: true,
            material,
            u: 0.0,
            v: 0.0,
        }
    }

    // Sets the normal facing towards ray origin
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }

    pub fn emitted(&self) -> Color3 {
        self.material.emitted(self.u, self.v, &self.position)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}
