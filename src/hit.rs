use crate::*;

pub struct HitRecord {
    pub position: Vec3,
    pub normal: Vec3,
    pub distance: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn at_position_and_distance(position: Vec3, distance: f64) -> Self {
        Self {
            position,
            normal: Vec3::zero(),
            distance: distance,
            front_face: true,
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
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
