use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}};
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let temp_rec = object.hit(ray, t_min, closest_so_far);
            if let Some(hit_record) = &temp_rec {
                closest_so_far = hit_record.t;
                record = temp_rec;
            }
        }
        record
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }
        let mut output_box = AABB::new();
        let mut first_box = true;

        for object in &self.objects {
            let temp_box = object.bounding_box(time_start, time_end)?;
            output_box = if first_box {
                temp_box.clone()
            } else {
                output_box.surrounding_box(&temp_box)
            };
            first_box = false;
        }
        Some(output_box)
    }
}
