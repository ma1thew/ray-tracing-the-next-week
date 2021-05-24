use std::sync::Arc;

use super::Material;
use crate::{hittable::HitRecord, texture::Texture, vec3::Vec3};
use crate::vec3::Color;
use crate::texture::SolidColor;
use crate::ray::Ray;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor { color_value: color.clone() }),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in : &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = &hit_record.normal + Vec3::random_unit_vector();

        // Catch zero-vector scatter directions that will generate issues later
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal.clone();
        }

        *scattered = Ray { origin: hit_record.p.clone(), direction: scatter_direction, time: ray_in.time };
        *attenuation = self.albedo.value(hit_record.u, hit_record.v, &hit_record.p);
        true
    }
}
