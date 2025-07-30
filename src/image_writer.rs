use crate::vector::*;

// origin: top left corner
// x: points to right
// y: points to bottom
pub struct ImageWriter {
    image: Vec<Vec<u32>>,
}

impl ImageWriter {
    pub fn new(width: usize, height: usize) -> ImageWriter {
        ImageWriter {
            image: vec![vec![0; width]; height],
        }
    }

    pub fn set_pixel_color(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let r = r as u32;
        let g = g as u32;
        let b = b as u32;
        let new_pixel = (r << 16) + (g << 8) + b;

        self.image[y][x] = new_pixel;
    }

    // NOTE: input vector should be normalized (each component is in [0, 1])
    pub fn set_pixel_color_vec(&mut self, x: usize, y: usize, color_vec: &Vector3d) {
        let r = (color_vec[0] * 255.0) as u32;
        let g = (color_vec[1] * 255.0) as u32;
        let b = (color_vec[2] * 255.0) as u32;
        let new_pixel = (r << 16) + (g << 8) + b;

        self.image[y][x] = new_pixel;
    }

    pub fn write_to_stdout(&self) {
        let (height, width) = (self.image.len(), self.image[0].len());

        println!("P3\n{width} {height}\n255\n");

        for y in 0..height {
            for x in 0..width {
                let pixel = self.image[y][x];
                let (r, g, b) = (pixel >> 16, (pixel >> 8) & 0xff, pixel & 0xff);
                println!("{r} {g} {b}");
            }
        }
    }
}
