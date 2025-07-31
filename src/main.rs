mod camera;
mod image_writer;
mod ray;
mod scene;
mod vector;

use rand::Rng;
use std::rc::Rc;

use image_writer::*;
use scene::*;
use vector::*;

use crate::scene::{
    drawable::sphere::Sphere,
    material::{
        Material, dielectric_material::DielectricMaterial, diffuse_material::DiffuseMaterial,
        metal_material::MetalMaterial,
    },
};

fn main() {
    let main_camera = camera::Camera::new(
        1920,
        90.0,
        16.0 / 9.0,
        Vector3d::new([0.0, 1.5, -1.5]),
        Vector3d::new([0.0, 0.4, -0.2]),
        256,
        32,
        0.0,
        1.0,
    );

    let mut main_scene = Scene::new();

    let diffuse_ground = Rc::new(DiffuseMaterial::new(Color::new([0.2, 0.2, 0.2])));

    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([0.0, -1000.0, 1.0]),
        1000.0,
        diffuse_ground.clone(),
    )));

    let mut rng = rand::rng();

    for i in -3..5 {
        for j in 0..5 {
            let material_choice = rng.random_range(0..10);
            let material: Rc<dyn Material> = if material_choice <= 5 {
                Rc::new(DiffuseMaterial::new(Color::random_unit_sphere()))
            } else if material_choice <= 7 {
                Rc::new(MetalMaterial::new(Color::random_unit_sphere(), 0.0))
            } else {
                Rc::new(DielectricMaterial::new(1.5))
            };

            let random_radius = rng.random_range(0.1..0.3);

            main_scene.add_object(Box::new(Sphere::new(
                Vector3d::new([
                    (i as f64) + rng.random_range(-0.5..0.5),
                    random_radius,
                    (j as f64) + rng.random_range(-0.5..0.5),
                ]),
                random_radius,
                material.clone(),
            )));
        }
    }

    main_camera.render(&main_scene);
}
