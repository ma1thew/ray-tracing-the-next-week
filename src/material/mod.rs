mod lambertian;
pub use lambertian::Lambertian;
mod metal;
pub use metal::Metal;
mod dielectric;
pub use dielectric::Dielectric;
mod diffuse_light;
pub use diffuse_light::DiffuseLight;
mod isotropic;
pub use isotropic::Isotropic;

use crate::{hittable::HitRecord, vec3::Point3};
use crate::vec3::Color;
use crate::ray::Ray;

pub trait Material: Send + Sync {
    fn emitted(&self, _: f64, _: f64, _: &Point3) -> Color {
        Color { x: 0.0, y: 0.0, z: 0.0 }    
    }
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
