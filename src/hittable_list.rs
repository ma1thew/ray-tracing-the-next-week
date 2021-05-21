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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self, time_start: f64, time_end: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box = AABB::new();
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(time_start, time_end, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                output_box.surrounding_box(&temp_box)
            };
            first_box = false;
        }
        true
    }
}
