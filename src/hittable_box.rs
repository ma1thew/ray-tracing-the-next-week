use std::sync::Arc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, hittable_list::HittableList, material::Material, ray::Ray, vec3::Point3, xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect};

pub struct HittableBox {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl HittableBox {
    pub fn new(min: Point3, max: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XYRect { material: material.clone(), x0: min.x, x1: max.x, y0: min.y, y1: max.y, k: max.z }));
        sides.add(Arc::new(XYRect { material: material.clone(), x0: min.x, x1: max.x, y0: min.y, y1: max.y, k: min.z }));

        sides.add(Arc::new(XZRect { material: material.clone(), x0: min.x, x1: max.x, z0: min.z, z1: max.z, k: max.y }));
        sides.add(Arc::new(XZRect { material: material.clone(), x0: min.x, x1: max.x, z0: min.z, z1: max.z, k: min.y }));

        sides.add(Arc::new(YZRect { material: material.clone(), y0: min.y, y1: max.y, z0: min.z, z1: max.z, k: max.x }));
        sides.add(Arc::new(YZRect { material: material.clone(), y0: min.y, y1: max.y, z0: min.z, z1: max.z, k: min.x }));

        Self {
            min,
            max,
            sides,
        }
    }
}

impl Hittable for HittableBox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB { minimum: self.min.clone(), maximum: self.max.clone() })
    }
}
