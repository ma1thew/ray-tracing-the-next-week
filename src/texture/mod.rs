mod perlin;
mod solid_color;
pub use solid_color::SolidColor;
mod checker_texture;
pub use checker_texture::CheckerTexture;
mod noise_texture;
pub use noise_texture::NoiseTexture;
mod image_texture;
pub use image_texture::ImageTexture;

use crate::{vec3::Color, vec3::Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

