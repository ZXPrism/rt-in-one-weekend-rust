mod image_writer;
mod vector;

use image_writer::ImageWriter;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut image_writer = ImageWriter::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            image_writer.set_pixel_color(
                x,
                y,
                ((x as f64) / (WIDTH as f64) * 255.0) as u8,
                ((y as f64) / (HEIGHT as f64) * 255.0) as u8,
                0,
            );
        }
    }

    image_writer.write_to_stdout();
}
