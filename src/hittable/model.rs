use std::sync::Arc;
use std::vec::Vec;

use crate::{hittable::{HitRecord, Hittable, AABB, HittableList, Triangle}, material::Material, ray::Ray, vec3::{Point3, Vec3}};

pub struct Model {
    faces: HittableList,
}

impl Model {
    fn parse_face_triplet(triplet: &str) -> Option<(isize, Option<isize>, Option<isize>)> {
        let mut triplet_iter = triplet.split("/");
        Some((triplet_iter.next()?.parse::<isize>().ok()?, triplet_iter.next().and_then(|val| val.parse::<isize>().ok()) , triplet_iter.next().and_then(|val| val.parse::<isize>().ok())))
    }
    pub fn from_obj(obj_data: &str, material: Arc<dyn Material>) -> Self {
        let mut geometric_vertices: Vec<Vec3> = Vec::new();
        let mut texture_vertices: Vec<(f64, f64)> = Vec::new();
        let mut vertex_normals: Vec<Vec3> = Vec::new();
        // no free-form objects, so no parameter-space vertices!
        let mut faces: Vec<Arc<dyn Hittable>> = Vec::new();
        for entry in obj_data.lines() {
            if entry.starts_with("#") {
                // comment
                continue;
            }
            let mut entry_iter = entry.split(' ');
            let operator = match entry_iter.next() {
                None => continue,
                Some(val) => val,
            };
            match operator {
                "v" | "vn" => {
                    let x = match entry_iter.next() {
                        None => {
                            eprintln!("Malformed {} entry in OBJ: Missing x!", operator);
                            continue;
                        },
                        Some(val) => {
                            match val.parse::<f64>() {
                                Err(_) => {
                                    eprintln!("Malformed {} entry in OBJ: Malformed f64 x!", operator);
                                    continue;
                                },
                                Ok(val) => val,
                            }
                        }
                    };
                    let y = match entry_iter.next() {
                        None => {
                            eprintln!("Malformed {} entry in OBJ: Missing y!", operator);
                            continue;
                        },
                        Some(val) => {
                            match val.parse::<f64>() {
                                Err(_) => {
                                    eprintln!("Malformed {} entry in OBJ: Malformed f64 y!", operator);
                                    continue;
                                },
                                Ok(val) => val,
                            }
                        }
                    };
                    let z = match entry_iter.next() {
                        None => {
                            eprintln!("Malformed {} entry in OBJ: Missing z!", operator);
                            continue;
                        },
                        Some(val) => {
                            match val.parse::<f64>() {
                                Err(_) => {
                                    eprintln!("Malformed {} entry in OBJ: Malformed f64 z!", operator);
                                    continue;
                                },
                                Ok(val) => val,
                            }
                        }
                    };
                    // who cares about w
                    match operator {
                        "v" => geometric_vertices.push(Vec3 {x, y, z}),
                        "vn" => vertex_normals.push(Vec3 {x, y, z}),
                        _ => panic!(),
                    }
                },
                "vt" => {
                    let u = match entry_iter.next() {
                        None => {
                            eprintln!("Malformed vt entry in OBJ: Missing u!");
                            continue;
                        },
                        Some(val) => {
                            match val.parse::<f64>() {
                                Err(_) => {
                                    eprintln!("Malformed vt entry in OBJ: Malformed f64 u!");
                                    continue;
                                },
                                Ok(val) => val,
                            }
                        }
                    };
                    let v = match entry_iter.next() {
                        None => {
                            eprintln!("Malformed vt entry in OBJ: Missing v!");
                            continue;
                        },
                        Some(val) => {
                            match val.parse::<f64>() {
                                Err(_) => {
                                    eprintln!("Malformed v entry in OBJ: Malformed f64 v!");
                                    continue;
                                },
                                Ok(val) => val,
                            }
                        }
                    };
                    // who cares about w
                    texture_vertices.push((u, v));
                },
                "f" => {
                    let mut triplets : Vec<(isize, Option<isize>, Option<isize>)> = Vec::new();
                    for triplet in entry_iter {
                        match Self::parse_face_triplet(triplet) {
                            None => {
                                eprintln!("Encountered malformed triplet in f operator!");
                            },
                            Some(val) => {
                                triplets.push(val);
                            }
                        };
                    }
                    // only support faces with *exactly* three vertices. yeah, i know.
                    if triplets.len() != 3 {
                        eprintln!("Encountered face with unsupported vertex count!");
                        continue;
                    }
                    let mut v0_index = triplets.get(0).unwrap().0;
                    if v0_index < 0 {
                        v0_index = geometric_vertices.len() as isize + v0_index;
                    } else {
                        v0_index = v0_index - 1;
                    }
                    let mut v1_index = triplets.get(1).unwrap().0;
                    if v1_index < 0 {
                        v1_index = geometric_vertices.len() as isize + v1_index;
                    } else {
                        v1_index = v1_index - 1;
                    }
                    let mut v2_index = triplets.get(2).unwrap().0;
                    if v2_index < 0 {
                        v2_index = geometric_vertices.len() as isize + v2_index;
                    } else {
                        v2_index = v2_index - 1;
                    }
                    let mut triangle = Triangle {
                        v0: geometric_vertices.get(v0_index as usize).unwrap().clone(),
                        v1: geometric_vertices.get(v1_index as usize).unwrap().clone(),
                        v2: geometric_vertices.get(v2_index as usize).unwrap().clone(),
                        material: material.clone(),
                        custom_normal: None,
                    };
                    if let Some(vn0) = triplets.get(0).unwrap().2 {
                        if let Some(vn1) = triplets.get(1).unwrap().2 {
                            if let Some(vn2) = triplets.get(2).unwrap().2 {
                                if vn0 != vn1 || vn1 != vn2 {
                                    eprintln!("Unsupported geometry in OBJ file: Multiple normals for face!");
                                    continue
                                }
                                let mut vn0 = vn0;
                                if vn0 < 0 {
                                    vn0 = vertex_normals.len() as isize + v0_index;
                                } else {
                                    vn0 = vn0 - 1;
                                }
                                triangle.custom_normal = Some(vertex_normals.get(vn0 as usize).unwrap().unit_vector());
                            }
                        }
                    }
                    faces.push(Arc::new(triangle));
                },
                _ => {
                    eprintln!("Ignoring unknown operator {} in OBJ!", operator);
                    continue;
                },
            }
        }
        Self {
            faces: HittableList { objects: faces },
        }
    }
}

impl Hittable for Model {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.faces.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        self.faces.bounding_box(time_start, time_end)
    }
}
