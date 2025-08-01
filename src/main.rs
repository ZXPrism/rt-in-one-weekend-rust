mod camera;
mod image_writer;
mod ray;
mod scene;
mod vector;

use rand::Rng;

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
    // tweak these parameters to adjust the pespective and output quality
    let main_camera = camera::Camera::new(
        1920,
        50.0,
        16.0 / 9.0,
        Vector3d::new([0.0, 1.2, -1.3]),
        Vector3d::new([0.0, 0.9, 0.0]),
        512,
        64,
        0.0,
        1.0,
    );

    let mut main_scene = Scene::new();

    let diffuse_ground = Box::new(DiffuseMaterial::new(Color::new([0.2, 0.2, 0.2])));

    main_scene.add_object(Box::new(Sphere::new(
        Vector3d::new([0.0, -1000.0, 1.0]),
        1000.0,
        diffuse_ground,
    )));

    let mut rng = rand::rng();

    for i in -3..5 {
        for j in 0..5 {
            let mut random_color = Color::random_unit_sphere();

            // correct negative elements
            random_color[0] = random_color[0].abs();
            random_color[1] = random_color[1].abs();
            random_color[2] = random_color[2].abs();

            // prevent too dark colors
            if random_color[0] < 0.2 {
                random_color[0] += 0.2;
            }
            if random_color[1] < 0.2 {
                random_color[1] += 0.2;
            }
            if random_color[2] < 0.2 {
                random_color[2] += 0.2;
            }

            let material_choice = rng.random_range(0..10);
            let material: Box<dyn Material> = if material_choice <= 5 {
                Box::new(DiffuseMaterial::new(random_color))
            } else if material_choice <= 7 {
                Box::new(MetalMaterial::new(random_color, 0.0))
            } else {
                Box::new(DielectricMaterial::new(1.5))
            };

            let random_radius = rng.random_range(0.1..0.35);

            main_scene.add_object(Box::new(Sphere::new(
                Vector3d::new([
                    (i as f64) + rng.random_range(-0.5..0.5),
                    random_radius,
                    (j as f64) + rng.random_range(-0.5..0.5),
                ]),
                random_radius,
                material,
            )));
        }
    }

    main_camera.render(&main_scene);
}
