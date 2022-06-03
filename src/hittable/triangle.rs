use std::sync::Arc;

use crate::{hittable::{HitRecord, Hittable, AABB}, material::Material, ray::Ray, vec3::{Point3, Vec3}};

pub struct Triangle {
    pub v0: Point3,
    pub v1: Point3,
    pub v2: Point3,
    pub material: Arc<dyn Material>,
    pub custom_normal: Option<Vec3>,
}

impl Triangle {
    fn has_vertex_at_infinity(&self) -> bool {
        self.v0.has_infinite_member() || self.v1.has_infinite_member() || self.v2.has_infinite_member()
    }
}

impl Hittable for Triangle {
    // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let epsilon: f64 = 0.0000001;
        let edge1 = &self.v1 - &self.v0;
        let edge2 = &self.v2 - &self.v0;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a > -epsilon && a < epsilon {
            return None; // This ray is parallel to the triangle.
        }
        let f = 1.0 / a;
        let s = &ray.origin - &self.v0;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this point, we can compute the point of intersection.
        let t = f * edge2.dot(&q);
        if t < t_min || t > t_max {
            return None;
        }
        let mut hit_record = HitRecord::new();
        hit_record.u = u;
        hit_record.v = v;
        hit_record.t = t;
        hit_record.p = ray.at(t);
        // TODO: i don't love this, but it allows for custom surface normals from OBJ data.
        if let Some(normal) = &self.custom_normal {
            hit_record.set_face_normal(ray, &normal);
        } else {
            let outward_normal = edge2.cross(&edge1).unit_vector();
            hit_record.set_face_normal(ray, &outward_normal);
        }
        hit_record.material = Some(self.material.clone());
        Some(hit_record)
    }
    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        match self.has_vertex_at_infinity() {
            true => None,
            false => Some(AABB {
                minimum: Point3 {
                    x: self.v0.x.min(self.v1.x).min(self.v2.x) - 0.0001,
                    y: self.v0.y.min(self.v1.y).min(self.v2.y) - 0.0001,
                    z: self.v0.z.min(self.v1.z).min(self.v2.z) - 0.0001,
                },
                maximum: Point3 {
                    x: self.v0.x.max(self.v1.x).max(self.v2.x) + 0.0001,
                    y: self.v0.y.max(self.v1.y).max(self.v2.y) + 0.0001,
                    z: self.v0.z.max(self.v1.z).max(self.v2.z) + 0.0001,                    
                },
            })
        }
    }
}
