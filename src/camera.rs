use crate::ImageWriter;
use crate::ray::{self, Ray};
use crate::scene::{self, Scene};
use crate::vector::Vector3d;

pub struct Camera {
    image_width: usize,
    image_height: usize,
    aspect_ratio: f64,

    center: Vector3d,
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
}

impl Camera {
    pub fn new(
        image_width: usize,
        viewport_width: f64,
        aspect_ratio: f64,
        camera_center: Vector3d,
        focal_length: f64,
    ) -> Self {
        let image_height = ((image_width as f64) / aspect_ratio) as usize;
        Camera {
            image_width,
            image_height,
            aspect_ratio,
            center: camera_center,
            viewport_width,
            viewport_height: ((viewport_width as f64) / aspect_ratio) as f64,
            focal_length,
        }
    }

    pub fn render(&self, scene: &Scene) {
        let viewport_u = Vector3d::new([self.viewport_width, 0.0, 0.0]);
        let viewport_v = Vector3d::new([0.0, -self.viewport_height, 0.0]);
        let viewport_u_delta = viewport_u / (self.image_width as f64);
        let viewport_v_delta = viewport_v / (self.image_height as f64);
        let viewport_upper_left = self.center + Vector3d::new([0.0, 0.0, self.focal_length])
            - (viewport_u + viewport_v) / 2.0;
        let pixel00 = viewport_upper_left - (viewport_u_delta + viewport_v_delta) / 2.0;

        let mut image_writer = ImageWriter::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let pixel = pixel00 + viewport_u_delta * (x as f64) + viewport_v_delta * (y as f64);
                let ray_direction = pixel - self.center;
                let ray_direction_norm = ray_direction.unit_vec();
                let primary_ray = Ray::new(self.center, ray_direction);

                let mut t = ray_direction_norm[1];
                t = (t + 1.0) * 0.5;

                let color =
                    Vector3d::new([1.0, 1.0, 1.0]) * (1.0 - t) + Vector3d::new([0.5, 0.7, 1.0]) * t;
                image_writer.set_pixel_color_vec(x, y, &color);
            }
        }

        image_writer.write_to_stdout();
    }
}
