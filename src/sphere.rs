use std::sync::Arc;
use std::f64::consts;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, material::Material, vec3::Vec3};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let phi = -p.z.atan2(p.x) + consts::PI;

        *u = phi / (2.0 * consts::PI);
        *v = theta / consts::PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies within acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal = (&hit_record.p - &self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        Self::get_sphere_uv(&outward_normal, &mut hit_record.u, &mut hit_record.v);
        hit_record.material = Some(self.material.clone());

        true
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            minimum: &self.center - Vec3 { x: self.radius, y: self.radius, z: self.radius },
            maximum: &self.center + Vec3 { x: self.radius, y: self.radius, z: self.radius },
        };
        true
    }
}
