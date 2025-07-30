mod camera;
mod image_writer;
mod ray;
mod scene;
mod vector;

use image_writer::*;
use scene::*;
use vector::*;

use crate::scene::drawable::sphere::Sphere;

fn main() {
    let main_camera = camera::Camera::new(
        400,
        90.0,
        16.0 / 9.0,
        Vector3d::new([0.0, 0.0, 0.0]),
        Vector3d::new([0.0, 0.0, 1.0]),
        128,
        32,
    );

    let mut main_scene = Scene::new();

    main_scene.add_object(Box::new(Sphere::new(Vector3d::new([0.0, 0.0, 1.0]), 0.5)));
    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([0.0, -100.5, 1.0]),
        100.0,
    )));

    main_camera.render(&main_scene);
}
