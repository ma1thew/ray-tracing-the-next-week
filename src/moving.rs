use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, ray::Ray, vec3::Vec3};

pub struct Moving {
    pub hittable: Arc<dyn Hittable>,
    pub offset_start: Vec3,
    pub offset_end: Vec3,
    pub time_start: f64,
    pub time_end: f64,
}

impl Moving {
    fn offset_at(&self, time: f64) -> Vec3 {
        &self.offset_start + ((time - self.time_start) / (self.time_end - self.time_start)) * (&self.offset_end - &self.offset_start)
    }
}

impl Hittable for Moving {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray { origin: &ray.origin - &self.offset_at(ray.time), direction: ray.direction.clone(), time: ray.time };
        let mut hit_record = self.hittable.hit(&moved_ray, t_min, t_max)?;
        hit_record.p += self.offset_at(ray.time).clone();
        let normal = hit_record.normal.clone();
        hit_record.set_face_normal(&moved_ray, &normal);
        Some(hit_record)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64, output_box: &mut AABB) -> bool {
        if !self.hittable.bounding_box(time_start, time_end, output_box) {
            return false;
        }

        *output_box = AABB {
            minimum: &output_box.minimum + &self.offset_at(time_start),
            maximum: &output_box.maximum + &self.offset_at(time_start),
        }.surrounding_box(&AABB {
            minimum: &output_box.minimum + &self.offset_at(time_end),
            maximum: &output_box.maximum + &self.offset_at(time_end),
        });
        true
    }
}
