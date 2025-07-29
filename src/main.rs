mod image_writer;
mod vector;

use image_writer::ImageWriter;
use vector::*;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut image_writer = ImageWriter::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = Vector3f::new([
                (x as f32) / (WIDTH as f32),
                (y as f32) / (HEIGHT as f32),
                0.0,
            ]);
            image_writer.set_pixel_color_vec(x, y, &color);
        }
    }

    image_writer.write_to_stdout();
}
