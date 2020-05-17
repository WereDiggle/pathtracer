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

pub struct FlipFace(pub Arc<dyn Hittable + Send + Sync>);

impl FlipFace {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>) -> Arc<Self> {
        Arc::new(Self(object))
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.0.hit(r, t_min, t_max).and_then(|hit| {
            Some(HitRecord {
                front_face: !hit.front_face,
                ..hit
            })
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.0.bounding_box(t0, t1)
    }
}

pub struct Translation {
    object: Arc<dyn Hittable + Send + Sync>,
    offset: Vec3,
}

impl Translation {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Arc<Self> {
        Arc::new(Self { object, offset })
    }
}

impl Hittable for Translation {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        self.object.hit(&moved_r, t_min, t_max).and_then(|mut hit| {
            hit.position += self.offset;
            hit.set_face_normal(&moved_r, hit.normal); // Redundant?
            Some(hit)
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.object
            .bounding_box(t0, t1)
            .and_then(|bbox| Some(AABB::new(bbox.min + self.offset, bbox.max + self.offset)))
    }
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub struct YRotation {
    object: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl YRotation {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, degrees: f64) -> Arc<Self> {
        let radians = degrees.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box(0.0, 1.0).and_then(|bbox| {
            let mut min = Vec3::infinity();
            let mut max = Vec3::neg_infinity();

            for x in [bbox.min.x(), bbox.max.x()].iter() {
                for y in [bbox.min.y(), bbox.max.y()].iter() {
                    for z in [bbox.min.z(), bbox.max.z()].iter() {
                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3(new_x, *y, new_z);

                        min = min.min(&tester);
                        max = max.max(&tester);
                    }
                }
            }

            Some(AABB::new(min, max))
        });

        Arc::new(Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        })
    }
}

impl Hittable for YRotation {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin[0] = self.cos_theta * r.origin[0] - self.sin_theta * r.origin[2];
        origin[2] = self.sin_theta * r.origin[0] + self.cos_theta * r.origin[2];

        direction[0] = self.cos_theta * r.direction[0] - self.sin_theta * r.direction[2];
        direction[2] = self.sin_theta * r.direction[0] + self.cos_theta * r.direction[2];

        let rotated_r = Ray::new(origin, direction, r.time);

        self.object
            .hit(&rotated_r, t_min, t_max)
            .and_then(|mut hit| {
                let mut position = hit.position;
                let mut normal = hit.normal;

                position[0] = self.cos_theta * hit.position[0] + self.sin_theta * hit.position[2];
                position[2] = -self.sin_theta * hit.position[0] + self.cos_theta * hit.position[2];

                normal[0] = self.cos_theta * hit.normal[0] + self.sin_theta * hit.normal[2];
                normal[2] = -self.sin_theta * hit.normal[0] + self.cos_theta * hit.normal[2];

                hit.position = position;
                hit.set_face_normal(&rotated_r, normal);

                Some(hit)
            })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.bbox.clone()
    }
}
