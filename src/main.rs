mod camera;
mod image_writer;
mod ray;
mod vector;

use image_writer::ImageWriter;
use vector::*;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut image_writer = ImageWriter::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = Vector3d::new([
                (x as f64) / (WIDTH as f64),
                (y as f64) / (HEIGHT as f64),
                0.0,
            ]);
            image_writer.set_pixel_color_vec(x, y, &color);
        }
    }

    image_writer.write_to_stdout();
}
