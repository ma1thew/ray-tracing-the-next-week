use std::io::Write;

use crate::vec3::Color;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
}

#[derive(Clone)]
struct Pixel {
    color: Color,
    sample_count: u32,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let data = vec![
            Pixel {
                color: Color {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                sample_count: 0
            };
            width * height
        ];
        Image {
            width,
            height,
            data,
        }
    }

    pub fn add_sample(&mut self, x: usize, y: usize, color: Color) {
        self.data
            .get_mut((y * self.width) + x)
            .unwrap()
            .update(color);
    }

    pub fn write(&self, output: &mut impl Write) {
        output.write_fmt(format_args!("P3\n{} {}\n255\n", self.width, self.height)).unwrap();
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = self.data.get((y * self.width) + x).unwrap();
                let mut r = pixel.color.x;
                let mut g = pixel.color.y;
                let mut b = pixel.color.z;

                // Divide by the number of samples and perform gamma correction for gamma 2
                let scale = 1.0 / pixel.sample_count as f64;
                r = (r * scale).sqrt();
                g = (g * scale).sqrt();
                b = (b * scale).sqrt();

                output
                    .write_fmt(format_args!(
                        "{} {} {}\n",
                        (256.0 * r.clamp(0.0, 0.999)) as u32,
                        (256.0 * g.clamp(0.0, 0.999)) as u32,
                        (256.0 * b.clamp(0.0, 0.999)) as u32,
                    ))
                    .unwrap();
            }
        }
    }
}

impl Pixel {
    pub fn update(&mut self, color: Color) {
        self.color += color;
        self.sample_count += 1;
    }
}
