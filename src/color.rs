use std::io::{Error, Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(output: &mut impl Write, pixel_color: Color, samples_per_pixel: u32) -> Result<(), Error> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide by the number of samples and perform gamma correction for gamma 2
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    output.write_fmt(format_args!(
        "{} {} {}\n",
        (256.0 * r.clamp(0.0, 0.999)) as u32,
        (256.0 * g.clamp(0.0, 0.999)) as u32,
        (256.0 * b.clamp(0.0, 0.999)) as u32,
    ))
}
