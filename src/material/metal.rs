use super::Material;
use crate::{hittable::HitRecord, vec3::Vec3};
use crate::vec3::Color;
use crate::ray::Ray;

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
