pub mod instance;
mod bvh_node;
pub use bvh_node::BVHNode;
mod constant_medium;
pub use constant_medium::ConstantMedium;
mod hittable_box;
pub use hittable_box::HittableBox;
mod hittable_list;
pub use hittable_list::HittableList;
mod xy_rect;
pub use xy_rect::XYRect;
mod xz_rect;
pub use xz_rect::XZRect;
mod yz_rect;
pub use yz_rect::YZRect;
mod sphere;
pub use sphere::Sphere;
mod triangle;
pub use triangle::Triangle;
mod aabb;

use std::sync::Arc;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;
use aabb::AABB;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Option<Arc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            material: None,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB>;
}
