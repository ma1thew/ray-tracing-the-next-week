use crate::vec3::{Vec3, Point3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranvec.push(Vec3::random_in_range(-1.0, 1.0).unit_vector());
        }
        Self {
            ranvec,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    pub fn turb(&self, point: &Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_point = point.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }
        accum.abs()
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();
        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = Default::default();
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self
                        .ranvec
                        .get(
                            (self.perm_x.get(((i + di as i32) & 255) as usize).unwrap()
                                ^ self.perm_y.get(((j + dj as i32) & 255) as usize).unwrap()
                                ^ self.perm_z.get(((k + dk as i32) & 255) as usize).unwrap())
                                as usize,
                        )
                        .unwrap().clone();
                }
            }
        }
        Self::trilinear_interpolate(c, u, v, w)
    }

    fn trilinear_interpolate(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum: f64 = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;
                    let weight_v = Vec3 { x: u - i_f, y: v - j_f, z: w - k_f };
                    accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu)) *
                        (j_f * vv + (1.0 - j_f) * (1.0 - vv)) *
                        (k_f * ww + (1.0 - k_f) * (1.0 - ww)) *
                        c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }

    fn generate_perm() -> Vec<usize> {
        let mut p = (0..POINT_COUNT).collect();
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in (1..n).rev() {
            p.swap(i, rand::random::<usize>() % i);
        }
    }
}
