use std::cmp::Ordering;

use rand::{thread_rng, Rng};

use crate::acceleration::hit_list;
use crate::*;

pub struct BVH {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bound_box: AABB,
}

impl BVH {
    pub fn from_hit_list(world: HitList, time: (f64, f64)) -> Self {
        Self::node(world.objects, time)
    }

    pub fn node(mut objects: Vec<Arc<dyn Hittable + Send + Sync>>, time: (f64, f64)) -> Self {
        let objects_box = hit_list::bounding_box(&objects, time).unwrap();
        let axis =
            if objects_box.max[0] - objects_box.min[0] > objects_box.max[1] - objects_box.min[1] {
                0
            } else {
                1
            };
        let axis = if objects_box.max[2] - objects_box.min[2]
            > objects_box.max[axis] - objects_box.min[axis]
        {
            2
        } else {
            axis
        };
        //let axis = thread_rng().gen_range(0, 3);

        objects.sort_by(|a, b| {
            let a_box = a
                .bounding_box(time.0, time.1)
                .expect("Object has bounding box");
            let b_box = b
                .bounding_box(time.0, time.1)
                .expect("Object has bounding box");

            if a_box.min[axis] == b_box.min[axis] {
                Ordering::Equal
            } else if a_box.min[axis] < b_box.min[axis] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let (left, right) = if objects.len() == 1 {
            (objects[0].clone(), objects[0].clone())
        } else if objects.len() == 2 {
            (objects[0].clone(), objects[1].clone())
        } else {
            let right_objects = objects.split_off(objects.len() / 2);
            let left: Arc<dyn Hittable + Send + Sync> = Arc::new(Self::node(objects, time));
            let right: Arc<dyn Hittable + Send + Sync> = Arc::new(Self::node(right_objects, time));
            (left, right)
        };

        let left_box = left
            .bounding_box(time.0, time.1)
            .expect("Object has bounding box");
        let right_box = right
            .bounding_box(time.0, time.1)
            .expect("Object has bounding box");

        let bound_box = left_box.surrounding_box(&right_box);
        Self {
            left,
            right,
            bound_box,
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bound_box.hit(r, t_min, t_max) {
            return None;
        }

        if let Some(hit) = self.left.hit(r, t_min, t_max) {
            self.right.hit(r, t_min, hit.distance).or(Some(hit))
        } else {
            self.right.hit(r, t_min, t_max)
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(self.bound_box.clone())
    }
}
