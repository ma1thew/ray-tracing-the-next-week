use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, ray::Ray, vec3::Vec3};

pub struct Translate {
    pub hittable: Arc<dyn Hittable>,
    pub offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray { origin: &ray.origin - &self.offset, direction: ray.direction.clone(), time: ray.time };
        let mut hit_record = self.hittable.hit(&moved_ray, t_min, t_max)?;
        hit_record.p += self.offset.clone();
        let normal = hit_record.normal.clone();
        hit_record.set_face_normal(&moved_ray, &normal);
        Some(hit_record)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64, output_box: &mut AABB) -> bool {
        if !self.hittable.bounding_box(time_start, time_end, output_box) {
            return false;
        }

        *output_box = AABB {
            minimum: &output_box.minimum + &self.offset,
            maximum: &output_box.maximum + &self.offset,
        };
        true
    }
}
