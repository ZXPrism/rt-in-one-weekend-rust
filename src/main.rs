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

use clap::Parser;

#[derive(Parser)]
#[command(name = "rt-in-one-weekend-rust")]
#[command(author = "ZXPrism")]
#[command(version = "1.0")]
#[command(about = "A rust implementation of the famous ray tracing tutorial \"Ray Tracing in One Weekend\"", long_about = None)]
struct CmdArgs {
    #[arg(long, default_value = "out.png", help = "output image file name")]
    output: String,

    #[arg(long, default_value_t = 800, help = "output image width")]
    width: usize,

    #[arg(long, default_value_t = 600, help = "output image height")]
    height: usize,

    #[arg(
        long,
        default_value_t = 32,
        help = "SPP (samples per pixel), significantly affects the output quality"
    )]
    spp: usize,

    #[arg(
        long,
        default_value_t = 32,
        help = "max bounce time (hit test recursion depth) of a single ray, affects the output quality"
    )]
    max_bounce: usize,

    #[arg(
        long,
        default_value_t = 50.0,
        help = "field of view, do not modify if you don't know what it is"
    )]
    fov: f64,

    #[arg(
        long,
        default_value_t = 0.0,
        help = "depth of field settings, do not modify if you don't know what it is"
    )]
    defocus_angle: f64,

    #[arg(
        long,
        default_value_t = 1.0,
        help = "depth of field settings, do not modify if you don't know what it is"
    )]
    focus_dist: f64,
}

fn main() {
    let args = CmdArgs::parse();

    let main_camera = camera::Camera::new(
        args.width,
        args.fov,
        (args.width as f64) / (args.height as f64),
        Vector3d::new([0.0, 1.2, -1.3]),
        Vector3d::new([0.0, 0.9, 0.0]),
        args.spp,
        args.max_bounce,
        args.defocus_angle,
        args.focus_dist,
        &args.output,
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
