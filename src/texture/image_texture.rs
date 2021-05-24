use super::Texture;
use crate::{vec3::Color, vec3::Point3};

// assume 24 bit depth
const BYTES_PER_PIXEL: usize = 3;
pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn from_bmp_data(bmp_data: &Vec<u8>) -> Self {
        let data_position = u32::from_le_bytes([
            bmp_data[0x0A],
            bmp_data[0x0B],
            bmp_data[0x0C],
            bmp_data[0x0D],
        ]);
        // assuming windows BITMAPINFOHEADER, these are i32
        let width = i32::from_le_bytes([
            bmp_data[0x12],
            bmp_data[0x13],
            bmp_data[0x14],
            bmp_data[0x15],
        ]) as usize;
        let height = i32::from_le_bytes([
            bmp_data[0x16],
            bmp_data[0x17],
            bmp_data[0x18],
            bmp_data[0x19],
        ]) as usize;
        Self {
            data: bmp_data[(data_position as usize)..bmp_data.len()].to_vec(),
            height,
            width,
            bytes_per_scanline: BYTES_PER_PIXEL * width,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        // This is a deviation from the book, where v gets flipped.
        // This is probably because the BMP loader loads in stuff upside down.
        //let v = 1.0 - v.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;

        if i >= self.width { i = self.width - 1 };
        if j >= self.height { j = self.height - 1 };
        let color_scale = 1.0 / 255.0;
        let pixel = j * self.bytes_per_scanline + i * BYTES_PER_PIXEL;
        Color {
            x: color_scale * *self.data.get(pixel + 2).unwrap() as f64,
            y: color_scale * *self.data.get(pixel + 1).unwrap() as f64,
            z: color_scale * *self.data.get(pixel).unwrap() as f64,
        }
    }
}
