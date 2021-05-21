use std::{cmp, sync::Arc};

use rand::seq::SliceRandom;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, hittable_list::HittableList};
use crate::ray::Ray;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb: AABB,
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl BVHNode {
    pub fn new(hittable_list: &HittableList, time_start: f64, time_end: f64) -> BVHNode {
        Self::from_objects(&hittable_list.objects, 0, hittable_list.objects.len(), time_start, time_end)
    }

    fn from_objects(src_objects: &Vec<Arc<dyn Hittable>>, start: usize, end: usize, time_start: f64, time_end: f64) -> BVHNode {
        let mut objects = src_objects.clone();
        let comparator = [
            |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| Self::box_compare(a.clone(), b.clone(), Axis::X),
            |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| Self::box_compare(a.clone(), b.clone(), Axis::Y),
            |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| Self::box_compare(a.clone(), b.clone(), Axis::Z),
        ].choose(&mut rand::thread_rng()).unwrap();
        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (objects.get(start).unwrap().clone(), objects.get(start).unwrap().clone()),
            2 => match comparator(objects.get(start).unwrap(), objects.get(start + 1).unwrap()) {
                    cmp::Ordering::Less => (objects.get(start).unwrap().clone(), objects.get(start + 1).unwrap().clone()),
                    _ => (objects.get(start + 1).unwrap().clone(), objects.get(start).unwrap().clone()),
                }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                (Arc::new(BVHNode::from_objects(&objects, start, mid, time_start, time_end)) as Arc<dyn Hittable>,
                    Arc::new(BVHNode::from_objects(&objects, mid, end, time_start, time_end)) as Arc<dyn Hittable>)

            }
        };
        let mut box_left = AABB::new();
        let mut box_right = AABB::new();
        if !left.bounding_box(time_start, time_end, &mut box_left) || !right.bounding_box(time_start, time_end, &mut box_right) {
            panic!("No bounding box in bvh_node constructor!")
        }

        BVHNode {
            left,
            right,
            aabb: box_left.surrounding_box(&box_right),
        }
    }

    fn box_compare (a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: Axis) -> cmp::Ordering {
        let mut box_a = AABB::new();
        let mut box_b = AABB::new();

        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in bvh_node constructor!")
        }
        // TODO: total_cmp is unstable :(
        box_a.minimum.get(axis as usize).unwrap().partial_cmp(box_b.minimum.get(axis as usize).unwrap()).unwrap()
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        if !self.aabb.hit(ray, t_min, t_max) {
            return false
        }
        let hit_left = self.left.hit(ray, t_min, t_max, hit_record);
        let hit_right = self.right.hit(ray, t_min, if hit_left { hit_record.t } else { t_max }, hit_record);
        hit_left || hit_right
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut AABB) -> bool {
        *output_box = self.aabb.clone();
        true
    }
}
