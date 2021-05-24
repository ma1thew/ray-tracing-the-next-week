use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction.get(a).unwrap();
            let mut t0 = (self.minimum.get(a).unwrap() - ray.origin.get(a).unwrap()) * inv_d;
            let mut t1 = (self.maximum.get(a).unwrap() - ray.origin.get(a).unwrap()) * inv_d;
            if inv_d < 0.0 {
                // TODO: destructuring assignments are unstable :(
                //(t0, t1) = (t1, t0);
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(&self, other: &AABB) -> AABB {
        AABB {
            minimum: Point3 {
                x: self.minimum.x.min(other.minimum.x),
                y: self.minimum.y.min(other.minimum.y),
                z: self.minimum.z.min(other.minimum.z),
            },
            maximum: Point3 {
                x: self.maximum.x.max(other.maximum.x),
                y: self.maximum.y.max(other.maximum.y),
                z: self.maximum.z.max(other.maximum.z),
            },
        }
    }
}
