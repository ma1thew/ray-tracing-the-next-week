use std::sync::Arc;

use crate::{hittable::{HitRecord, Hittable, AABB}, material::Material, ray::Ray, vec3::{Point3, Vec3}};

pub struct XZRect {
    pub material: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl XZRect {
    fn has_infinite_bounds(&self) -> bool {
        self.x0.is_infinite() || self.x1.is_infinite() || self.z0.is_infinite() || self.z1.is_infinite()
    }
}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut hit_record = HitRecord::new();
        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (z - self.z0) / (self.z1 - self.z0);
        hit_record.t = t;
        let outward_normal = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material = Some(self.material.clone());
        hit_record.p = ray.at(t);
        Some(hit_record)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        match self.has_infinite_bounds() {
            true => None,
            false => Some(AABB { minimum: Point3 { x: self.x0, y: self.k - 0.0001, z: self.z0 }, maximum: Point3 { x: self.x1, y: self.k + 0.0001, z: self.z1 } }),
        }
    }
}
