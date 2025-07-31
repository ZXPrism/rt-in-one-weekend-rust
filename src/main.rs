mod camera;
mod image_writer;
mod ray;
mod scene;
mod vector;

use std::rc::Rc;

use image_writer::*;
use scene::*;
use vector::*;

use crate::scene::{
    drawable::sphere::Sphere, material::diffuse_material::DiffuseMaterial,
    material::metal_material::MetalMaterial,
};

fn main() {
    let main_camera = camera::Camera::new(
        800,
        90.0,
        16.0 / 9.0,
        Vector3d::new([0.0, 0.0, -1.0]),
        Vector3d::new([0.0, 0.0, 1.0]),
        128,
        32,
    );

    let mut main_scene = Scene::new();

    let diffuse = Rc::new(DiffuseMaterial::new(Color::new([0.7, 0.7, 0.5])));
    let metal = Rc::new(MetalMaterial::new(Color::new([0.7, 0.7, 0.5])));

    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([0.0, 0.0, 1.0]),
        0.5,
        diffuse.clone(),
    )));
    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([0.0, -100.5, 1.0]),
        100.0,
        diffuse.clone(),
    )));
    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([-1.2, 0.0, 1.0]),
        0.5,
        metal.clone(),
    )));
    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([1.2, 0.0, 1.0]),
        0.5,
        metal.clone(),
    )));

    main_camera.render(&main_scene);
}
