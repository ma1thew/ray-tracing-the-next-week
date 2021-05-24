use std::{cmp, sync::Arc};

use rand::seq::SliceRandom;

use crate::hittable::{HitRecord, Hittable, AABB, hittable_list::HittableList};
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
        let box_left = left.bounding_box(time_start, time_end).expect("No bounding box in bvh_node constructor!");
        let box_right = right.bounding_box(time_start, time_end).expect("No bounding box in bvh_node constructor!");

        BVHNode {
            left,
            right,
            aabb: box_left.surrounding_box(&box_right),
        }
    }

    fn box_compare (a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: Axis) -> cmp::Ordering {
        let box_a = a.bounding_box(0.0, 0.0).expect("No bounding box in bvh_node constructor!");
        let box_b = b.bounding_box(0.0, 0.0).expect("No bounding box in bvh_node constructor!");

        // TODO: total_cmp is unstable :(
        box_a.minimum.get(axis as usize).unwrap().partial_cmp(box_b.minimum.get(axis as usize).unwrap()).unwrap()
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None
        }
        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right_threshold = if let Some(hit_record_left) = &hit_left {
            hit_record_left.t
        } else {
            t_max
        };
        let hit_right = self.right.hit(ray, t_min, hit_right_threshold);
        if let Some(_) = &hit_right {
            hit_right
        } else {
            hit_left
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}
