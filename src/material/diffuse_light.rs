use std::sync::Arc;

use super::Material;
use crate::{hittable::HitRecord, texture::Texture, vec3::Point3};
use crate::vec3::Color;
use crate::texture::SolidColor;
use crate::ray::Ray;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_color(color: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::from_color(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Color, _: &mut Ray) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, point: &Point3) -> Color {
        self.emit.value(u, v, point)
    }
}
