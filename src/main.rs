mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;
mod image;
mod aabb;
mod bvh_node;
mod texture;
mod perlin;
mod xy_rect;
mod xz_rect;
mod yz_rect;
mod hittable_box;
mod translate;
mod rotate_y;
mod constant_medium;
mod moving;

use std::{sync::{Arc, mpsc::{self, Sender}}, thread};

use camera::Camera;
use constant_medium::ConstantMedium;
use hittable::{HitRecord, Hittable};
use hittable_box::HittableBox;
use hittable_list::HittableList;
use image::Image;
use material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use ray::Ray;
use rotate_y::RotateY;
use sphere::Sphere;
use translate::Translate;
use vec3::{Point3, Vec3, Color};
use bvh_node::BVHNode;
use texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use xy_rect::XYRect;
use xz_rect::XZRect;
use yz_rect::YZRect;

use crate::moving::Moving;

struct PixelUpdate {
    color: Color,
    x: usize,
    y: usize,
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_colors(Color { x: 0.2, y: 0.3, z: 0.1 } , Color { x: 0.9, y: 0.9, z: 0.9 }));
    let ground_material = Arc::new(Lambertian { albedo: checker });
    world.add(Arc::new(Sphere{ center: Point3 { x: 0.0, y: -1000.0, z: 0.0 }, radius: 1000.0, material: ground_material }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3 { x: (a as f64) + 0.9 * rand::random::<f64>(), y: 0.2, z: (b as f64) + 0.9 * rand::random::<f64>() };

            if (&center - Point3 { x: 4.0, y: 0.3, z: 0.0 }).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Arc::new(SolidColor::from_color(Color::random() * Color::random()));
                    sphere_material = Arc::new(Lambertian { albedo });
                    world.add(Arc::new(Sphere { center, radius: 0.2, material: sphere_material }));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>() / 2.0;
                    sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.add(Arc::new(Sphere { center, radius: 0.2, material: sphere_material }));
                } else {
                    sphere_material = Arc::new(Dielectric { index_of_refraction: 1.5 });
                    world.add(Arc::new(Sphere { center, radius: 0.2, material: sphere_material }));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric { index_of_refraction: 1.5 });
    world.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 1.0, z: 0.0 }, radius: 1.0, material: material1 }));

    let material2 = Arc::new(Lambertian { albedo: Arc::new(SolidColor::from_color(Color { x: 0.4, y: 0.2, z: 0.1 })) });
    world.add(Arc::new(Sphere { center: Point3 { x: -4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: material2 }));

    let material3 = Arc::new(Metal { albedo: Color { x: 0.7, y: 0.6, z: 0.5 }, fuzz: 0.0 });
    world.add(Arc::new(Sphere { center: Point3 { x: 4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: material3 }));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = Arc::new(Lambertian { albedo: Arc::new(CheckerTexture::from_colors(Color { x: 0.2, y: 0.3, z: 0.1 } , Color { x: 0.9, y: 0.9, z: 0.9 })) });
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: -10.0, z: 0.0 }, radius: 10.0, material: checker.clone() }));
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 10.0, z: 0.0 }, radius: 10.0, material: checker }));
    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(Lambertian { albedo: Arc::new(NoiseTexture::new(4.0)) });
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: -1000.0, z: 0.0 }, radius: 1000.0, material: pertext.clone() }));
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 2.0, z: 0.0 }, radius: 2.0, material: pertext.clone() }));
    objects
}

fn earth() -> HittableList {
    let mut objects = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::from_bmp_data(&include_bytes!("../res/earthmap.bmp").to_vec()));
    let earth_surface = Arc::new(Lambertian { albedo: earth_texture });
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 0.0, z: 0.0 }, radius: 2.0, material: earth_surface }));
    objects
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(Lambertian { albedo: Arc::new(NoiseTexture::new(4.0)) });
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: -1000.0, z: 0.0 }, radius: 1000.0, material: pertext.clone() }));
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 2.0, z: 0.0 }, radius: 2.0, material: pertext.clone() }));
    let diff_light = Arc::new(DiffuseLight::from_color(Color { x: 4.0, y: 4.0, z: 4.0 }));
    objects.add(Arc::new(XYRect { material: diff_light, x0: 3.0, x1: 5.0, y0: 1.0, y1: 3.0, k: -2.0 }));
    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian::from_color(Color { x: 0.65, y: 0.05, z: 0.05 }));
    let white = Arc::new(Lambertian::from_color(Color { x: 0.73, y: 0.73, z: 0.73 }));
    let green = Arc::new(Lambertian::from_color(Color { x: 0.12, y: 0.45, z: 0.15 }));
    let light = Arc::new(DiffuseLight::from_color(Color { x: 15.0, y: 15.0, z: 15.0 }));

    objects.add(Arc::new(YZRect { material: green, y0: 0.0, y1: 555.0, z0: 0.0, z1: 555.0, k: 555.0 }));
    objects.add(Arc::new(YZRect { material: red, y0: 0.0, y1: 555.0, z0: 0.0, z1: 555.0, k: 0.0 }));
    objects.add(Arc::new(XZRect { material: light, x0: 213.0, x1: 343.0, z0: 227.0, z1: 332.0, k: 554.0 }));
    objects.add(Arc::new(XZRect { material: white.clone(), x0: 0.0, x1: 555.0, z0: 0.0, z1: 555.0, k: 0.0 }));
    objects.add(Arc::new(XZRect { material: white.clone(), x0: 0.0, x1: 555.0, z0: 0.0, z1: 555.0, k: 555.0 }));
    objects.add(Arc::new(XYRect { material: white.clone(), x0: 0.0, x1: 555.0, y0: 0.0, y1: 555.0, k: 555.0 }));

    //objects.add(Arc::new(HittableBox::new(Point3 { x: 130.0, y: 0.0, z: 65.0 }, Point3 { x: 295.0, y: 165.0, z: 230.0 }, white.clone())));
    //objects.add(Arc::new(HittableBox::new(Point3 { x: 265.0, y: 0.0, z: 295.0 }, Point3 { x: 430.0, y: 330.0, z: 460.0 }, white.clone())));
    let box_1 = Arc::new(HittableBox::new(Point3 { x: 0.0, y: 0.0, z: 0.0 }, Point3 { x: 165.0, y: 330.0, z: 165.0 }, white.clone()));
    let box_1 = Arc::new(RotateY::new(box_1, 15.0));
    let box_1 = Arc::new(Translate { hittable: box_1, offset: Point3 { x: 265.0, y: 0.0, z: 295.0 } });
    objects.add(box_1);
    let box_2 = Arc::new(HittableBox::new(Point3 { x: 0.0, y: 0.0, z: 0.0 }, Point3 { x: 165.0, y: 165.0, z: 165.0 }, white.clone()));
    let box_2 = Arc::new(RotateY::new(box_2, -18.0));
    let box_2 = Arc::new(Translate { hittable: box_2, offset: Point3 { x: 130.0, y: 0.0, z: 65.0 } });
    objects.add(box_2);

    objects
}

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian::from_color(Color { x: 0.65, y: 0.05, z: 0.05 }));
    let white = Arc::new(Lambertian::from_color(Color { x: 0.73, y: 0.73, z: 0.73 }));
    let green = Arc::new(Lambertian::from_color(Color { x: 0.12, y: 0.45, z: 0.15 }));
    let light = Arc::new(DiffuseLight::from_color(Color { x: 7.0, y: 7.0, z: 7.0 }));

    objects.add(Arc::new(YZRect { material: green, y0: 0.0, y1: 555.0, z0: 0.0, z1: 555.0, k: 555.0 }));
    objects.add(Arc::new(YZRect { material: red, y0: 0.0, y1: 555.0, z0: 0.0, z1: 555.0, k: 0.0 }));
    objects.add(Arc::new(XZRect { material: light, x0: 113.0, x1: 443.0, z0: 127.0, z1: 432.0, k: 554.0 }));
    objects.add(Arc::new(XZRect { material: white.clone(), x0: 0.0, x1: 555.0, z0: 0.0, z1: 555.0, k: 0.0 }));
    objects.add(Arc::new(XZRect { material: white.clone(), x0: 0.0, x1: 555.0, z0: 0.0, z1: 555.0, k: 555.0 }));
    objects.add(Arc::new(XYRect { material: white.clone(), x0: 0.0, x1: 555.0, y0: 0.0, y1: 555.0, k: 555.0 }));

    //objects.add(Arc::new(HittableBox::new(Point3 { x: 130.0, y: 0.0, z: 65.0 }, Point3 { x: 295.0, y: 165.0, z: 230.0 }, white.clone())));
    //objects.add(Arc::new(HittableBox::new(Point3 { x: 265.0, y: 0.0, z: 295.0 }, Point3 { x: 430.0, y: 330.0, z: 460.0 }, white.clone())));
    let box_1 = Arc::new(HittableBox::new(Point3 { x: 0.0, y: 0.0, z: 0.0 }, Point3 { x: 165.0, y: 330.0, z: 165.0 }, white.clone()));
    let box_1 = Arc::new(RotateY::new(box_1, 15.0));
    let box_1 = Arc::new(Translate { hittable: box_1, offset: Point3 { x: 265.0, y: 0.0, z: 295.0 } });
    let box_2 = Arc::new(HittableBox::new(Point3 { x: 0.0, y: 0.0, z: 0.0 }, Point3 { x: 165.0, y: 165.0, z: 165.0 }, white.clone()));
    let box_2 = Arc::new(RotateY::new(box_2, -18.0));
    let box_2 = Arc::new(Translate { hittable: box_2, offset: Point3 { x: 130.0, y: 0.0, z: 65.0 } });

    objects.add(Arc::new(ConstantMedium::new(box_1, 0.01, Arc::new(SolidColor::from_color(Color { x: 0.0, y: 0.0, z: 0.0 })))));
    objects.add(Arc::new(ConstantMedium::new(box_2, 0.01, Arc::new(SolidColor::from_color(Color { x: 1.0, y: 1.0, z: 1.0 })))));

    objects
}

fn final_scene() -> HittableList {
    let mut boxes_1 = HittableList::new();
    let ground = Arc::new(Lambertian::from_color(Color { x: 0.48, y: 0.83, z: 0.53 }));

    const BOXES_PER_SIDE: usize = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 1.0 + 100.0 * rand::random::<f64>();
            let z1 = z0 + w;
            boxes_1.add(Arc::new(HittableBox::new(Point3 { x: x0, y: y0, z: z0 }, Point3 { x: x1, y: y1, z: z1  }, ground.clone())));
        }
    }

    let mut objects = HittableList::new();
    objects.add(Arc::new(BVHNode::new(&boxes_1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::from_color(Color { x: 7.0, y: 7.0, z: 7.0 }));
    objects.add(Arc::new(XZRect { material: light.clone(), x0: 123.0, x1: 423.0, z0: 147.0, z1: 412.0, k: 554.0 }));

    let center_1 = Point3 { x: 400.0, y: 400.0, z: 200.0 };
    let center_2 = &center_1 + Vec3 { x: 30.0, y: 0.0, z: 0.0 };

    let moving_sphere_material = Arc::new(Lambertian::from_color(Color { x: 0.7, y: 0.3, z: 0.1 }));
    objects.add(Arc::new(Moving { hittable: Arc::new(Sphere { center: Point3::new(), radius: 50.0, material: moving_sphere_material }), offset_start: center_1, offset_end: center_2, time_start: 0.0, time_end: 1.0, }));

    objects.add(Arc::new(Sphere { center: Point3 { x: 260.0, y: 150.0, z: 45.0 }, radius: 50.0, material: Arc::new(Dielectric { index_of_refraction: 1.5 }) }));
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 150.0, z: 145.0 }, radius: 50.0, material: Arc::new(Metal { albedo: Color { x: 0.8, y: 0.8, z: 0.9 }, fuzz: 1.0 }) }));

    let boundary = Arc::new(Sphere { center: Point3 { x: 360.0, y: 150.0, z: 145.0 }, radius: 70.0, material: Arc::new(Dielectric { index_of_refraction: 1.5 }) });
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new(boundary.clone(), 0.2, Arc::new(SolidColor::from_color(Color { x: 0.2, y: 0.4, z: 0.9 })))));
    let boundary = Arc::new(Sphere { center: Point3 { x: 0.0, y: 0.0, z: 0.0 }, radius: 5000.0, material: Arc::new(Dielectric { index_of_refraction: 1.5 }) });
    objects.add(Arc::new(ConstantMedium::new(boundary.clone(), 0.0001, Arc::new(SolidColor::from_color(Color { x: 1.0, y: 1.0, z: 1.0 })))));

    let emat = Arc::new(Lambertian { albedo: Arc::new(ImageTexture::from_bmp_data(&include_bytes!("../res/earthmap.bmp").to_vec())) });
    objects.add(Arc::new(Sphere { center: Point3 { x: 400.0, y: 200.0, z: 400.0 }, radius: 100.0, material: emat }));
    let pertext = Arc::new(Lambertian { albedo: Arc::new(NoiseTexture::new(0.1)) });
    objects.add(Arc::new(Sphere { center: Point3 { x: 220.0, y: 280.0, z: 300.0 }, radius: 80.0, material: pertext }));

    let mut boxes_2 = HittableList::new();
    let white = Arc::new(Lambertian::from_color(Color { x: 0.73, y: 0.73, z: 0.73 }));
    for _ in 0..1000 {
        boxes_2.add(Arc::new(Sphere { center: Point3::random_in_range(0.0, 165.0), radius: 10.0, material: white.clone() }));
    }

    objects.add(Arc::new(Translate { hittable: Arc::new(RotateY::new(Arc::new(BVHNode::new(&boxes_2, 0.0, 1.0)), 15.0)), offset: Vec3 { x: -100.0, y: 270.0, z: 395.0 } }));
    objects
}

fn test_scene() -> HittableList {
    let mut objects = HittableList::new();
    //let earth_texture = Arc::new(ImageTexture::from_bmp_data(&include_bytes!("../res/earthmap.bmp").to_vec()));
    //let earth_surface = Arc::new(Lambertian { albedo: earth_texture });
    let pertext = Arc::new(Lambertian { albedo: Arc::new(NoiseTexture::new(4.0)) });
    objects.add(Arc::new(XZRect { material: pertext, x0: -f64::INFINITY, x1: f64::INFINITY, z0: -f64::INFINITY, z1: f64::INFINITY, k: 0.0 }));
    objects
}

fn ray_color(ray: &Ray, background: &Color, world: &dyn Hittable, depth: u32) -> Color {
    let mut rec = HitRecord::new();
    if depth <= 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    if !world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        background.clone()
    } else {
        let mut scattered = Ray::new();
        let mut attenuation = Color::new();
        if let Some(material) = &rec.material {
            let emitted = material.emitted(rec.u, rec.v, &rec.p);
            if !material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
                emitted
            } else {
                emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
            }
        } else {
            Color { x: 0.0, y: 0.0, z: 0.0 }
        }
    }
}

fn render(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, world: Arc<dyn Hittable>, background: Color, camera: Arc<Camera>, tx: Sender<PixelUpdate>) {
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rand::random::<f64>()) / ((image_width - 1) as f64);
                let v = ((j as f64) + rand::random::<f64>()) / ((image_height - 1) as f64);
                let ray = camera.get_ray(u, v);

                tx.send(PixelUpdate { color: ray_color(&ray, &background, world.as_ref(), max_depth), x: i as usize, y: j as usize}).unwrap();
            }
        }
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    //const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;
    const THREAD_COUNT: u32 = 8;
    const TIME_START: f64 = 0.0;
    const TIME_END: f64 = 1.0;
    // World
    let mut world: Arc<dyn Hittable> = Arc::new(BVHNode::new(&random_scene(), TIME_START, TIME_END));
    let mut lookfrom = Point3 { x: 13.0, y: 2.0, z: 3.0 };
    let mut lookat = Point3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut vfov = 20.0;
    let mut aperture = 0.1;
    let mut background = Color { x: 0.7, y: 0.8, z: 1.0 };

    match std::env::args().nth(1) { 
        Some(string) => { 
            match string.trim().parse::<u32>() {
            Ok(2) => {
                world = Arc::new(BVHNode::new(&two_spheres(), TIME_START, TIME_END));
                lookfrom = Point3 { x: 13.0, y: 2.0, z: 3.0 };
                lookat = Point3 { x: 0.0, y: 0.0, z: 0.0 };
                vfov = 20.0;
                aperture = 0.0;
                background = Color { x: 0.7, y: 0.8, z: 1.0 };
            },
            Ok(3) => {
                world = Arc::new(BVHNode::new(&two_perlin_spheres(), TIME_START, TIME_END));
                lookfrom = Point3 { x: 13.0, y: 2.0, z: 3.0 };
                lookat = Point3 { x: 0.0, y: 0.0, z: 0.0 };
                vfov = 20.0;
                aperture = 0.0;
            },
            Ok(4) => {
                world = Arc::new(BVHNode::new(&earth(), TIME_START, TIME_END));
                lookfrom = Point3 { x: 13.0, y: 2.0, z: 3.0 };
                lookat = Point3 { x: 0.0, y: 0.0, z: 0.0 };
                vfov = 20.0;
                aperture = 0.0;
            },
            Ok(5) => {
                world = Arc::new(BVHNode::new(&simple_light(), TIME_START, TIME_END));
                lookfrom = Point3 { x: 26.0, y: 3.0, z: 6.0 };
                lookat = Point3 { x: 0.0, y: 2.0, z: 0.0 };
                vfov = 20.0;
                aperture = 0.0;
                background = Color { x: 0.0, y: 0.0, z: 0.0 };
            },
            Ok(6) => {
                world = Arc::new(BVHNode::new(&cornell_box(), TIME_START, TIME_END));
                lookfrom = Point3 { x: 278.0, y: 278.0, z: -800.0 };
                lookat = Point3 { x: 278.0, y: 278.0, z: 0.0 };
                vfov = 40.0;
                aperture = 0.0;
                background = Color { x: 0.0, y: 0.0, z: 0.0 };
            },
            Ok(7) => {
                world = Arc::new(BVHNode::new(&cornell_smoke(), TIME_START, TIME_END));
                lookfrom = Point3 { x: 278.0, y: 278.0, z: -800.0 };
                lookat = Point3 { x: 278.0, y: 278.0, z: 0.0 };
                vfov = 40.0;
                aperture = 0.0;
                background = Color { x: 0.0, y: 0.0, z: 0.0 };
            },
            Ok(8) => {
                world = Arc::new(final_scene());
                lookfrom = Point3 { x: 478.0, y: 278.0, z: -600.0 };
                lookat = Point3 { x: 278.0, y: 278.0, z: 0.0 };
                vfov = 40.0;
                aperture = 0.0;
                background = Color { x: 0.0, y: 0.0, z: 0.0 };
            },
            Ok(9) => {
                world = Arc::new(test_scene());
                //world = Arc::new(BVHNode::new(&test_scene(), TIME_START, TIME_END));
                lookfrom = Point3 { x: 13.0, y: 4.0, z: 3.0 };
                lookat = Point3 { x: 0.0, y: 0.0, z: 0.0 };
                vfov = 20.0;
                aperture = 0.0;
            },
            _ => {},
        }},
        _ => {},
    }
    // Camera
    let vup = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    let dist_to_focus = 10.0;
    let cam = Arc::new(Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, TIME_START, TIME_END));
    // Render
    let mut final_image = Image::new(IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize);
    let (tx, rx) = mpsc::channel::<PixelUpdate>();
    for _ in 0..THREAD_COUNT {
        let sender = tx.clone();
        let world_ref = world.clone();
        let camera_ref = cam.clone();
        let background_clone = background.clone();
        thread::spawn( || {
            render(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL / THREAD_COUNT, MAX_DEPTH, world_ref, background_clone, camera_ref, sender);
        });
    }
    let expected_updates: u64 = (SAMPLES_PER_PIXEL / THREAD_COUNT) as u64 * THREAD_COUNT as u64 * IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64;
    let print_frequency: u64 = (SAMPLES_PER_PIXEL / THREAD_COUNT) as u64 * THREAD_COUNT as u64 * IMAGE_WIDTH as u64;
    let mut update_count: u64 = 0;
    loop {
        if let Ok(update) = rx.try_recv() {
            update_count += 1;
            final_image.add_sample(update.x, update.y, update.color);
            if update_count % print_frequency == 0 {
                eprintln!("Current completion: {:.2}%", (update_count as f64 / expected_updates as f64) * 100.0)
            }
        } else {
            if Arc::strong_count(&world) == 1 {
                break
            }
        }
    }
    final_image.write(&mut std::io::stdout());
    eprintln!("Done.");
}
