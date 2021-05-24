use std::sync::Arc;

use super::{Texture, SolidColor};
use crate::{vec3::Color, vec3::Point3};

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn from_colors(odd: Color, even: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::from_color(odd)),
            even: Arc::new(SolidColor::from_color(even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
