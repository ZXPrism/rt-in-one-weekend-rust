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
        let r: u32 = r.into();
        let g: u32 = g.into();
        let b: u32 = b.into();
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
