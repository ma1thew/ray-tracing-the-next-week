use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Point3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y:0.0, z: 0.0},
            time: 0.0
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + t * &self.direction
    }
}
