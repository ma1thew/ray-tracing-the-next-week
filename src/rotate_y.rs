use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, ray::Ray, util::degrees_to_radians, vec3::{Point3, Vec3}};

pub struct RotateY {
    hittable: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    has_box: bool,
    aabb: AABB,
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut aabb = AABB::new();
        let has_box = hittable.bounding_box(0.0, 1.0, &mut aabb); // TODO: passing in 0.0 and 1.0 for time seems suspicious.

        let mut min = Point3 { x: f64::INFINITY, y: f64::INFINITY, z: f64::INFINITY };
        let mut max = Point3 { x: -f64::INFINITY, y: -f64::INFINITY, z: -f64::INFINITY };
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * aabb.maximum.x + (1.0 - i as f64) * aabb.minimum.x;
                    let y = j as f64 * aabb.maximum.y + (1.0 - j as f64) * aabb.minimum.y;
                    let z = k as f64 * aabb.maximum.z + (1.0 - k as f64) * aabb.minimum.z;
                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3 { x: new_x, y, z: new_z };
                    for c in 0..3 {
                        *min.get_mut(c).unwrap() = min.get(c).unwrap().min(*tester.get(c).unwrap());
                        *max.get_mut(c).unwrap() = max.get(c).unwrap().max(*tester.get(c).unwrap());
                    }
                }
            }
        }
        aabb = AABB { minimum: min, maximum: max };

        Self {
            hittable,
            sin_theta,
            cos_theta,
            has_box,
            aabb,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin.clone();
        let mut direction = ray.direction.clone();

        origin.x = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin.z = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;
        
        direction.x = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        direction.z = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;

        let rotated_ray = Ray { origin, direction, time: ray.time };
        let mut hit_record = self.hittable.hit(&rotated_ray, t_min, t_max)?;

        let mut p = hit_record.p.clone();
        let mut normal = hit_record.normal.clone();

        p.x = self.cos_theta * hit_record.p.x + self.sin_theta * hit_record.p.z;
        p.z = -self.sin_theta * hit_record.p.x + self.cos_theta * hit_record.p.z;

        normal.x = self.cos_theta * hit_record.normal.x + self.sin_theta * hit_record.normal.z;
        normal.z = -self.sin_theta * hit_record.normal.x + self.cos_theta * hit_record.normal.z;

        hit_record.p = p;
        hit_record.set_face_normal(&rotated_ray, &normal);
        Some(hit_record)
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        *output_box = self.aabb.clone();
        self.has_box
    }
}
