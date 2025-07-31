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
    drawable::sphere::Sphere,
    material::{
        dielectric_material::DielectricMaterial, diffuse_material::DiffuseMaterial,
        metal_material::MetalMaterial,
    },
};

fn main() {
    let main_camera = camera::Camera::new(
        800,
        90.0,
        16.0 / 9.0,
        Vector3d::new([0.0, 1.0, -1.0]),
        Vector3d::new([0.0, 0.0, 0.0]),
        128,
        32,
    );

    let mut main_scene = Scene::new();

    let diffuse = Rc::new(DiffuseMaterial::new(Color::new([0.1, 0.2, 0.5])));
    let diffuse_ground = Rc::new(DiffuseMaterial::new(Color::new([0.8, 0.8, 0.0])));
    let metal = Rc::new(MetalMaterial::new(Color::new([0.7, 0.7, 0.5]), 0.0));
    let dielectric = Rc::new(DielectricMaterial::new(1.5));

    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([0.0, 0.0, 1.0]),
        0.5,
        diffuse.clone(),
    )));
    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([0.0, -100.5, 1.0]),
        100.0,
        diffuse_ground.clone(),
    )));
    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([-1.1, 0.1, 0.3]),
        0.6,
        metal.clone(),
    )));
    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([1.2, 0.0, 1.0]),
        0.5,
        dielectric.clone(),
    )));

    main_camera.render(&main_scene);
}
