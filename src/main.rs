// origin: top left corner
// x: points to right
// y: points to bottom
struct ImageWriter {
    image: Vec<Vec<u32>>,
}

impl ImageWriter {
    fn new(width: usize, height: usize) -> ImageWriter {
        ImageWriter {
            image: vec![vec![0; width]; height],
        }
    }

    fn set_pixel_color(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let r: u32 = r.into();
        let g: u32 = g.into();
        let b: u32 = b.into();
        let new_pixel = (r << 16) + (g << 8) + b;

        self.image[y][x] = new_pixel;
    }

    fn write_to_stdout(&self) {
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

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

mod vector;

fn main() {
    let mut image_writer = ImageWriter::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            image_writer.set_pixel_color(
                x,
                y,
                ((x as f64) / (WIDTH as f64) * 255.0) as u8,
                ((y as f64) / (WIDTH as f64) * 255.0) as u8,
                0,
            );
        }
    }

    image_writer.write_to_stdout();
}
