use std::sync::Arc;

use crate::{hittable::HitRecord, texture::Texture, vec3::{Point3, Vec3}};
use crate::vec3::Color;
use crate::texture::SolidColor;
use crate::ray::Ray;

pub trait Material: Send + Sync {
    fn emitted(&self, _: f64, _: f64, _: &Point3) -> Color {
        Color { x: 0.0, y: 0.0, z: 0.0 }    
    }
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

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

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64, // TODO: This should have value 0.0 - 1.0; this is not enforced
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = ray_in.direction.unit_vector().reflect(&hit_record.normal);
        *scattered = Ray { origin: hit_record.p.clone(), direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(), time: ray_in.time };
        *attenuation = self.albedo.clone();
        scattered.direction.dot(&hit_record.normal) > 0.0
    }
}

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Using Schlick's Approximation:
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color { x: 1.0, y: 1.0, z: 1.0 };
        let refraction_ratio = if hit_record.front_face { 1.0 / self.index_of_refraction } else { self.index_of_refraction };
        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = hit_record.normal.dot(&-&unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>() {
            direction = unit_direction.reflect(&hit_record.normal)
        } else {
            direction = unit_direction.refract(&hit_record.normal, refraction_ratio)
        }

        *scattered = Ray { origin: hit_record.p.clone(), direction, time: ray_in.time };
        true
    }
}

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
