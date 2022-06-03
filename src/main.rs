mod camera;
mod hittable;
mod material;
mod ray;
mod util;
mod vec3;
mod image;
mod texture;
mod scenes;

use std::{sync::{Arc, mpsc}, thread};

use camera::Camera;
use hittable::Hittable;
use image::Image;
use ray::Ray;
use vec3::{Vec3, Color};
use scenes::get_scene;

struct PixelUpdate {
    color: Color,
    x: usize,
    y: usize,
}

fn ray_color(ray: &Ray, background: &Color, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    match world.hit(ray, 0.001, f64::INFINITY) {
        None => background.clone(),
        Some(rec) => {
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
        },
    }
}

fn render(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, world: Arc<dyn Hittable>, background: Color, camera: Arc<Camera>, tx: mpsc::Sender<PixelUpdate>) {
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
    const SAMPLES_PER_PIXEL: u32 = 30;
    const MAX_DEPTH: u32 = 50;
    const THREAD_COUNT: u32 = 8;
    const TIME_START: f64 = 0.0;
    const TIME_END: f64 = 1.0;
    // World
    let (world, lookfrom, lookat, vfov, aperture, background) = get_scene(std::env::args().nth(1).unwrap_or("0".to_string()).trim().parse().unwrap_or(0));

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
                eprint!("\rCurrent completion: {:.2}%", (update_count as f64 / expected_updates as f64) * 100.0)
            }
        } else {
            if Arc::strong_count(&world) == 1 {
                break
            }
        }
    }
    final_image.write(&mut std::io::stdout());
    eprintln!("\nDone.");
}
