use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, material::Material, ray::Ray, vec3::{Point3, Vec3}};

pub struct XYRect {
    pub material: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (y - self.y0) / (self.y1 - self.y0);
        hit_record.t = t;
        let outward_normal = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material = Some(self.material.clone());
        hit_record.p = ray.at(t);
        true
    }

    fn bounding_box(&self, time_start: f64, time_end: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB { minimum: Point3 { x: self.x0, y: self.y0, z: self.k - 0.0001 }, maximum: Point3 { x: self.x1, y: self.y1, z: self.k + 0.0001 } };
        true
    }
}
