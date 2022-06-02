use std::sync::Arc;

use crate::hittable::{ConstantMedium, Hittable};
use crate::hittable::HittableBox;
use crate::hittable::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::hittable::instance::RotateY;
use crate::hittable::Sphere;
use crate::hittable::instance::Translate;
use crate::vec3::{Point3, Vec3, Color};
use crate::hittable::BVHNode;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use crate::hittable::XYRect;
use crate::hittable::XZRect;
use crate::hittable::YZRect;
use crate::hittable::Triangle;
use crate::hittable::instance::Moving;

pub fn get_scene(id: u32) -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
    match id {
        2 => two_spheres(),
        3 => two_perlin_spheres(),
        4 => earth(),
        5 => simple_light(),
        6 => cornell_box(),
        7 => cornell_smoke(),
        8 => final_scene(),
        9 => test_scene(),
        10 => triangle_scene(),
        _ => random_scene(),
    }
}

fn random_scene() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
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

    (Arc::new(BVHNode::new(&world, 0.0, 1.0)), Point3 { x: 13.0, y: 2.0, z: 3.0}, Point3::new(), 20.0, 0.1, Color { x: 0.7, y: 0.8, z: 1.0 })
}

fn two_spheres() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
    let mut objects = HittableList::new();
    let checker = Arc::new(Lambertian { albedo: Arc::new(CheckerTexture::from_colors(Color { x: 0.2, y: 0.3, z: 0.1 } , Color { x: 0.9, y: 0.9, z: 0.9 })) });
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: -10.0, z: 0.0 }, radius: 10.0, material: checker.clone() }));
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 10.0, z: 0.0 }, radius: 10.0, material: checker }));
    (Arc::new(BVHNode::new(&objects, 0.0, 1.0)), Point3 { x: 13.0, y: 2.0, z: 3.0}, Point3::new(), 20.0, 0.0, Color { x: 0.7, y: 0.8, z: 1.0 })
}

fn two_perlin_spheres() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
    let mut objects = HittableList::new();
    let pertext = Arc::new(Lambertian { albedo: Arc::new(NoiseTexture::new(4.0)) });
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: -1000.0, z: 0.0 }, radius: 1000.0, material: pertext.clone() }));
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 2.0, z: 0.0 }, radius: 2.0, material: pertext.clone() }));
    (Arc::new(BVHNode::new(&objects, 0.0, 1.0)), Point3 { x: 13.0, y: 2.0, z: 3.0}, Point3::new(), 20.0, 0.0, Color { x: 0.7, y: 0.8, z: 1.0 })
}

fn earth() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
    let mut objects = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::from_bmp_data(&include_bytes!("../res/earthmap.bmp").to_vec()));
    let earth_surface = Arc::new(Lambertian { albedo: earth_texture });
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 0.0, z: 0.0 }, radius: 2.0, material: earth_surface }));
    (Arc::new(BVHNode::new(&objects, 0.0, 1.0)), Point3 { x: 13.0, y: 2.0, z: 3.0}, Point3::new(), 20.0, 0.0, Color { x: 0.7, y: 0.8, z: 1.0 })
}

fn simple_light() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
    let mut objects = HittableList::new();
    let pertext = Arc::new(Lambertian { albedo: Arc::new(NoiseTexture::new(4.0)) });
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: -1000.0, z: 0.0 }, radius: 1000.0, material: pertext.clone() }));
    objects.add(Arc::new(Sphere { center: Point3 { x: 0.0, y: 2.0, z: 0.0 }, radius: 2.0, material: pertext.clone() }));
    let diff_light = Arc::new(DiffuseLight::from_color(Color { x: 4.0, y: 4.0, z: 4.0 }));
    objects.add(Arc::new(XYRect { material: diff_light, x0: 3.0, x1: 5.0, y0: 1.0, y1: 3.0, k: -2.0 }));
    (Arc::new(BVHNode::new(&objects, 0.0, 1.0)), Point3 { x: 26.0, y: 3.0, z: 6.0}, Point3 { x: 0.0, y: 2.0, z: 0.0}, 20.0, 0.0, Color::new())
}

fn cornell_box() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
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

    (Arc::new(BVHNode::new(&objects, 0.0, 1.0)), Point3 { x: 278.0, y: 278.0, z: -800.0}, Point3 { x: 278.0, y: 278.0, z: 0.0}, 40.0, 0.0, Color::new())
}

fn cornell_smoke() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
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

    (Arc::new(BVHNode::new(&objects, 0.0, 1.0)), Point3 { x: 278.0, y: 278.0, z: -800.0}, Point3 { x: 278.0, y: 278.0, z: 0.0}, 40.0, 0.0, Color::new())
}

fn final_scene() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
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
    (Arc::new(objects), Point3 { x: 478.0, y: 278.0, z: -600.0}, Point3 { x: 278.0, y: 278.0, z: 0.0}, 40.0, 0.0, Color::new())
}

fn test_scene() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
    let mut objects = HittableList::new();
    //let earth_texture = Arc::new(ImageTexture::from_bmp_data(&include_bytes!("../res/earthmap.bmp").to_vec()));
    //let earth_surface = Arc::new(Lambertian { albedo: earth_texture });
    let pertext = Arc::new(Lambertian { albedo: Arc::new(NoiseTexture::new(4.0)) });
    objects.add(Arc::new(XZRect { material: pertext, x0: -f64::INFINITY, x1: f64::INFINITY, z0: -f64::INFINITY, z1: f64::INFINITY, k: 0.0 }));
    //(Arc::new(BVHNode::new(&objects, 0.0, 1.0)), Point3 { x: 13.0, y: 2.0, z: 3.0}, Point3::new(), 20.0, 0.0, Color { x: 0.7, y: 0.8, z: 1.0 })
    (Arc::new(objects), Point3 { x: 13.0, y: 2.0, z: 3.0}, Point3::new(), 20.0, 0.0, Color { x: 0.7, y: 0.8, z: 1.0 })
}

fn triangle_scene() -> (Arc<dyn Hittable>, Point3, Point3, f64, f64, Color) {
    let mut objects = HittableList::new();
    //let pertext = Arc::new(Lambertian { albedo: Arc::new(NoiseTexture::new(4.0)) });
    let pertext = Arc::new(Lambertian { albedo: Arc::new(SolidColor::from_color(Color {x: 0.0, y: 0.0, z: 0.0}) ) });
    objects.add(Arc::new(Triangle { material: pertext, v0 : Vec3::new(), v1: Vec3 { x: 0.0, y: 0.0, z: 1.0 }, v2: Vec3 { x: 0.0, y: 1.0, z: 0.5 }  }));    
    (Arc::new(BVHNode::new(&objects, 0.0, 1.0)), Point3 { x: 5.0, y: 5.0, z: 5.0}, Point3::new(), 20.0, 0.0, Color { x: 0.7, y: 0.8, z: 1.0 })
}
