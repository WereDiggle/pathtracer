use crate::ray::*;
use crate::vec3::*;

pub struct HitRecord {
    pub t: f64,
    pub position: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
