mod camera;
mod image_writer;
mod ray;
mod scene;
mod vector;

use image_writer::*;
use scene::*;
use vector::*;

fn main() {
    let main_camera =
        camera::Camera::new(400, 4.0, 16.0 / 9.0, Vector3d::new([1.0, 1.0, 1.0]), 1.0);

    let mut main_scene = Scene::new();

    main_camera.render(&main_scene);
}
