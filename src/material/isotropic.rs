use std::sync::Arc;

use super::Material;
use crate::{hittable::HitRecord, texture::Texture, vec3::Vec3};
use crate::vec3::Color;
use crate::texture::SolidColor;
use crate::ray::Ray;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from_color(color)),
        }
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self {
            albedo: texture,
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *scattered = Ray { origin: hit_record.p.clone(), direction: Vec3::random_in_unit_sphere(), time: ray_in.time };
        *attenuation = self.albedo.value(hit_record.u, hit_record.v, &hit_record.p);
        true
    }
}
