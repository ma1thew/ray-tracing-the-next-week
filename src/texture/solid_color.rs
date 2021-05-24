use super::Texture;
use crate::{vec3::Color, vec3::Point3};

pub struct SolidColor {
    pub color_value: Color,
}

impl SolidColor {
    pub fn from_color(color_value: Color) -> Self {
        Self {
            color_value,
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: &Point3) -> Color {
        self.color_value.clone()
    }
}
