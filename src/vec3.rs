use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::fmt;

pub type Point3 = Vec3;

#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn get(&self, index: usize) -> Option<&f64> {
        match index {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            _ => None,
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: min + (max - min) * rand::random::<f64>(),
            y: min + (max - min) * rand::random::<f64>(),
            z: min + (max - min) * rand::random::<f64>(),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3 {
                x: 2.0 * rand::random::<f64>() - 1.0,
                y: 2.0 * rand::random::<f64>() - 1.0,
                z: 2.0 * rand::random::<f64>() - 1.0,
            };
            if p.length_squared() < 1.0 {
                return p;
            }
        };
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3 {
                x: 2.0 * rand::random::<f64>() -1.0,
                y: 2.0 * rand::random::<f64>() -1.0,
                z: 0.0,
            };
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = normal.dot(&-self).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
        r_out_perp + r_out_parallel
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl_op_ex!(- |a: &Vec3| -> Vec3 {
    Vec3 {
        x: -a.x,
        y: -a.y,
        z: -a.z,
    }
});
impl_op_ex!(+= |lhs: &mut Vec3, rhs: Vec3| { *lhs = Vec3 { x: lhs.x + rhs.x, y: lhs.y + rhs.y, z: lhs.z + rhs.z } });
impl_op_ex!(*= |lhs: &mut Vec3, rhs: &f64| { *lhs = Vec3 { x: lhs.x * rhs, y: lhs.y * rhs, z: lhs.z * rhs } });
impl_op_ex!(/= |lhs: &mut Vec3, rhs: &f64| { *lhs *= 1.0 / rhs });
impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3 { x: lhs.x + rhs.x, y: lhs.y + rhs.y, z: lhs.z + rhs.z } });
impl_op_ex!(-|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3 {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
});
impl_op_ex!(*|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3 {
        x: lhs.x * rhs.x,
        y: lhs.y * rhs.y,
        z: lhs.z * rhs.z,
    }
});
impl_op_ex_commutative!(*|lhs: &Vec3, rhs: &f64| -> Vec3 {
    Vec3 {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
        z: lhs.z * rhs,
    }
});
impl_op_ex!(/ |lhs: &Vec3, rhs: &f64| -> Vec3 { lhs * (1.0/rhs) });
