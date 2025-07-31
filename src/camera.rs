use rand::Rng;
use rayon::prelude::*;

use crate::ImageWriter;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::{Color, Vector3d};

pub struct Camera {
    image_width: usize,
    image_height: usize,
    camera_center: Vector3d,
    gaze_center: Vector3d,
    fov: f64,
    sample_cnt: usize,
    max_bounce: usize,

    // depth-of-field settings
    defocus_angle: f64,
    focus_dist: f64,
}

impl Camera {
    pub fn new(
        image_width: usize,
        fov: f64,
        aspect_ratio: f64,
        camera_center: Vector3d,
        gaze_center: Vector3d, // NOTE: not near plane! just a helper plane to delineate the frustum
        sample_cnt: usize,
        max_bounce: usize,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        Camera {
            image_width,
            image_height: ((image_width as f64) / aspect_ratio) as usize,
            camera_center,
            gaze_center,
            fov,
            sample_cnt,
            max_bounce,
            defocus_angle,
            focus_dist,
        }
    }

    pub fn render(&self, scene: &Scene) {
        let camera_gaze = self.gaze_center - self.camera_center;
        let camera_gaze_norm = camera_gaze.unit_vec();

        let camera_right = Vector3d::cross_product(camera_gaze, Vector3d::new([0.0, 1.0, 0.0]));
        let camera_right_norm = camera_right.unit_vec();

        let camera_down = Vector3d::cross_product(camera_gaze, camera_right);
        let camera_down_norm = camera_down.unit_vec();

        let viewport_height = 2.0 * (self.fov / 2.0).to_radians().tan() * self.focus_dist;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        let viewport_u = camera_right.unit_vec() * viewport_width;
        let viewport_v = camera_down.unit_vec() * viewport_height;
        let viewport_u_delta = viewport_u / (self.image_width as f64);
        let viewport_v_delta = viewport_v / (self.image_height as f64);
        let viewport_upper_left = self.camera_center + camera_gaze_norm * self.focus_dist
            - (viewport_u + viewport_v) / 2.0;
        let pixel00 = viewport_upper_left + (viewport_u_delta + viewport_v_delta) * 0.5;

        let defocus_radius = (self.defocus_angle / 2.0).to_radians().tan() * self.focus_dist;
        let defocus_disk_u = camera_right_norm * defocus_radius;
        let defocus_disk_v = camera_down_norm * defocus_radius;

        let mut image_writer = ImageWriter::new(self.image_width, self.image_height);

        let pixels: Vec<(usize, usize, Color)> = (0..self.image_height)
            .into_par_iter()
            .flat_map(|y| {
                let mut rng = rand::rng();
                let mut pixel = pixel00 + viewport_v_delta * (y as f64);

                (0..self.image_width)
                    .map(move |x| {
                        let mut color = Vector3d::zeros();

                        for _ in 0..self.sample_cnt {
                            let sample_pixel = pixel
                                + viewport_u_delta * rng.random_range(-0.5..=0.5)
                                + viewport_v_delta * rng.random_range(-0.5..=0.5);

                            let random_disk = Vector3d::random_unit_disk();
                            let defocus_origin = self.camera_center
                                + defocus_disk_u * random_disk[0]
                                + defocus_disk_v * random_disk[1];

                            let primary_ray =
                                Ray::new(defocus_origin, sample_pixel - self.camera_center);

                            color += self.hit_test(&primary_ray, scene, self.max_bounce);
                        }

                        color /= self.sample_cnt as f64;

                        pixel += viewport_u_delta;

                        (x, y, color)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        pixels.iter().for_each(|(x, y, color)| {
            image_writer.set_pixel_color_vec(*x, *y, color);
        });

        image_writer.write_to_file();
    }

    fn hit_test(&self, primary_ray: &Ray, scene: &Scene, remaining_bounce: usize) -> Color {
        if remaining_bounce == 0 {
            return Color::zeros();
        }

        let hit_info = scene.hit_test(&primary_ray);
        if hit_info.if_hit {
            self.hit_test(&hit_info.scatter_ray, scene, remaining_bounce - 1) * hit_info.albedo
        } else {
            let mut t = primary_ray.direction.unit_vec()[1];
            t = (t + 1.0) * 0.5;
            Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
        }
    }
}
