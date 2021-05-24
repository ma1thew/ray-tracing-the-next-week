use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, material::Material, ray::Ray, vec3::{Point3, Vec3}};

pub struct YZRect {
    pub material: Arc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl YZRect {
    fn has_infinite_bounds(&self) -> bool {
        self.y0.is_infinite() || self.y1.is_infinite() || self.z0.is_infinite() || self.z1.is_infinite()
    }
}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return false;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        hit_record.u = (y - self.y0) / (self.y1 - self.y0);
        hit_record.v = (z - self.z0) / (self.z1 - self.z0);
        hit_record.t = t;
        let outward_normal = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material = Some(self.material.clone());
        hit_record.p = ray.at(t);
        true
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        if self.has_infinite_bounds() {
            false
        } else {
            *output_box = AABB { minimum: Point3 { x: self.k - 0.0001, y: self.y0, z: self.z0 }, maximum: Point3 { x: self.k + 0.0001, y: self.y1, z: self.z1 } };
            true
        }
    }
}
