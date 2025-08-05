use image::{ImageBuffer, Rgb};

use crate::vector::*;

// origin: top left corner
// x: points to right
// y: points to bottom
pub struct ImageWriter {
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl ImageWriter {
    pub fn new(width: usize, height: usize) -> ImageWriter {
        ImageWriter {
            image: ImageBuffer::new(width as u32, height as u32),
        }
    }

    // NOTE: input vector should be normalized (each component is in [0, 1])
    pub fn set_pixel_color_vec(&mut self, x: usize, y: usize, color_vec: &Vector3d) {
        let mut color_vec = color_vec.clone();

        color_vec[0] = color_vec[0].sqrt();
        color_vec[1] = color_vec[1].sqrt();
        color_vec[2] = color_vec[2].sqrt();

        let r = (color_vec[0] * 255.0) as u8;
        let g = (color_vec[1] * 255.0) as u8;
        let b = (color_vec[2] * 255.0) as u8;

        self.image.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
    }

    pub fn write_to_file(&self, output_file_name: &str) {
        self.image.save(output_file_name).unwrap();
    }
}
