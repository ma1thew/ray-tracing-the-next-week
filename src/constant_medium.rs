use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, material::{Isotropic, Material}, ray::Ray, texture::Texture, vec3::Vec3};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::from_texture(texture)),
            neg_inv_density: -1.0/density,
        }
    }
}

impl Hittable for ConstantMedium {
    // TODO: this only support convex shapes.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut record_1 = HitRecord::new();
        let mut record_2 = HitRecord::new();

        if !self.boundary.hit(ray, -f64::INFINITY, f64::INFINITY, &mut record_1) {
            return false;
        }

        if !self.boundary.hit(ray, record_1.t + 0.0001, f64::INFINITY, &mut record_2) {
            return false;
        }

        if record_1.t < t_min {
            record_1.t = t_min;
        }
        if record_2.t > t_max {
            record_2.t = t_max;
        }

        if record_1.t >= record_2.t {
            return false;
        }

        if record_1.t < 0.0 {
            record_1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (record_2.t - record_1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return false
        }

        hit_record.t = record_1.t + hit_distance / ray_length;
        hit_record.p = ray.at(hit_record.t);
        hit_record.normal = Vec3 { x: 1.0, y: 0.0, z: 0.0 }; // arbitrary
        hit_record.front_face = true; // arbitrary
        hit_record.material = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self, time_start: f64, time_end: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time_start, time_end, output_box)
    }
}
