use std::sync::Arc;

use crate::*;

pub struct HitList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HitList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut final_hit_record: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.distance;
                final_hit_record = Some(hit_record);
            }
        }

        final_hit_record
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        bounding_box(&self.objects, (t0, t1))
    }
}

pub fn bounding_box(
    objects: &Vec<Arc<dyn Hittable + Send + Sync>>,
    time: (f64, f64),
) -> Option<AABB> {
    if objects.is_empty() {
        return None;
    }

    let mut final_box: Option<AABB> = None;

    for object in objects.iter() {
        if let Some(new_box) = object.bounding_box(time.0, time.1) {
            if let Some(cur_box) = final_box {
                final_box = Some(cur_box.surrounding_box(&new_box));
            } else {
                final_box = Some(new_box);
            }
        } else {
            return None;
        }
    }
    final_box
}
