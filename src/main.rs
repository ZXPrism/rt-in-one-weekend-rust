mod camera;
mod image_writer;
mod ray;
mod vector;

use image_writer::ImageWriter;
use vector::*;

fn main() {
    let main_camera = camera::Camera::new(100, 2.0, 16.0 / 9.0, 1.0);
    main_camera.render();
}
