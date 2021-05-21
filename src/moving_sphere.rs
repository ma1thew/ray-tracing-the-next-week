use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, material::Material, sphere::Sphere, vec3::Vec3};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct MovingSphere {
    pub center_start: Point3,
    pub center_end: Point3,
    pub time_start: f64,
    pub time_end: f64,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl MovingSphere {
    fn center(&self, time: f64) -> Point3 {
        &self.center_start + ((time - self.time_start) / (self.time_end - self.time_start)) * (&self.center_end - &self.center_start)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = &ray.origin - self.center(ray.time);
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
        let outward_normal = (&hit_record.p - self.center(ray.time)) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut hit_record.u, &mut hit_record.v);
        hit_record.material = Some(self.material.clone());

        true
    }

    fn bounding_box(&self, time_start: f64, time_end: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            minimum: self.center(time_start) - Vec3 { x: self.radius, y: self.radius, z: self.radius },
            maximum: self.center(time_start) + Vec3 { x: self.radius, y: self.radius, z: self.radius },
        }.surrounding_box(&AABB {
            minimum: self.center(time_end) - Vec3 { x: self.radius, y: self.radius, z: self.radius },
            maximum: self.center(time_end) + Vec3 { x: self.radius, y: self.radius, z: self.radius },
        });
        true
    }
}
