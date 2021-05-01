mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;
mod image;

use std::{io, rc::Rc, sync::{Arc, mpsc::{self, Sender}}, thread};

use camera::Camera;
use color::{write_color, Color};
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use image::Image;
use material::{Lambertian, Material, Metal, Dielectric};
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

struct PixelUpdate {
    color: Color,
    x: usize,
    y: usize,
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian { albedo: Color { x: 0.5, y: 0.5, z: 0.5 } });
    world.add(Arc::new(Sphere{ center: Point3 { x: 0.0, y: -1000.0, z: 0.0 }, radius: 1000.0, material: ground_material }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3 { x: (a as f64) + 0.9 * rand::random::<f64>(), y: 0.2, z: (b as f64) + 0.9 * rand::random::<f64>() };

            if (&center - Point3 { x: 4.0, y: 0.3, z: 0.0 }).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
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

    let material2 = Arc::new(Lambertian { albedo: Color { x: 0.4, y: 0.2, z: 0.1 } });
    world.add(Arc::new(Sphere { center: Point3 { x: -4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: material2 }));

    let material3 = Arc::new(Metal { albedo: Color { x: 0.7, y: 0.6, z: 0.5 }, fuzz: 0.0 });
    world.add(Arc::new(Sphere { center: Point3 { x: 4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: material3 }));

    world
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut rec = HitRecord::new();
    if depth <= 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Color::new();
        if let Some(material) = &rec.material {
            if material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
            return Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t)
        * Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

fn render(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, world: Arc<HittableList>, camera: Arc<Camera>, tx: Sender<PixelUpdate>) {
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rand::random::<f64>()) / ((image_width - 1) as f64);
                let v = ((j as f64) + rand::random::<f64>()) / ((image_height - 1) as f64);
                let ray = camera.get_ray(u, v);

                tx.send(PixelUpdate { color: ray_color(&ray, world.as_ref(), max_depth), x: i as usize, y: j as usize}).unwrap();
            }
        }
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;
    const THREAD_COUNT: u32 = 8;
    // World
    let world = Arc::new(random_scene());

    // Camera
    let lookfrom = Point3 { x: 13.0, y: 2.0, z: 3.0 };
    let lookat = Point3 { x: 0.0, y: 0.0, z: 0.0 };
    let vup = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Arc::new(Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus));
    // Render
    let mut final_image = Image::new(IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize);
    let mut thread_pool = Vec::with_capacity(THREAD_COUNT as usize);
    let (tx, rx) = mpsc::channel::<PixelUpdate>();
    for _ in 0..THREAD_COUNT {
        let sender = tx.clone();
        let world_ref = world.clone();
        let camera_ref = cam.clone();
        thread_pool.push(thread::spawn( || {
            render(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL / THREAD_COUNT, MAX_DEPTH, world_ref, camera_ref, sender);
        }));
    }
    while Arc::strong_count(&world) > 1 {
        let update = rx.recv().unwrap();
        final_image.add_sample(update.x, update.y, update.color);
    }
    final_image.write(&mut std::io::stdout());
    eprintln!("Done.");
}
