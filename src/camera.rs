use crate::ImageWriter;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vector3d;

pub struct Camera {
    image_width: usize,
    image_height: usize,
    camera_center: Vector3d,
    gaze_center: Vector3d,
    fov: f64,
}

impl Camera {
    pub fn new(
        image_width: usize,
        fov: f64,
        aspect_ratio: f64,
        camera_center: Vector3d,
        gaze_center: Vector3d,
    ) -> Self {
        Camera {
            image_width,
            image_height: ((image_width as f64) / aspect_ratio) as usize,
            camera_center,
            gaze_center,
            fov,
        }
    }

    pub fn render(&self, scene: &Scene) {
        let camera_gaze = self.gaze_center - self.camera_center;
        let camera_right = Vector3d::cross_product(camera_gaze, Vector3d::new([0.0, 1.0, 0.0]));
        let camera_down = Vector3d::cross_product(camera_gaze, camera_right);

        let focal_length = camera_gaze.length();
        let viewport_height = 2.0 * (self.fov / 2.0).to_radians().tan() * focal_length;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        let viewport_u = camera_right.unit_vec() * viewport_width;
        let viewport_v = camera_down.unit_vec() * viewport_height;
        let viewport_u_delta = viewport_u / (self.image_width as f64);
        let viewport_v_delta = viewport_v / (self.image_height as f64);
        let viewport_upper_left = self.gaze_center - (viewport_u + viewport_v) / 2.0;
        let pixel00 = viewport_upper_left + (viewport_u_delta + viewport_v_delta) * 0.5;

        let mut image_writer = ImageWriter::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            let mut pixel = pixel00 + viewport_v_delta * (y as f64);

            for x in 0..self.image_width {
                let ray_direction = pixel - self.camera_center;
                let ray_direction_norm = ray_direction.unit_vec();
                let primary_ray = Ray::new(self.camera_center, ray_direction);

                let hit_info = scene.hit_test(&primary_ray);

                let mut t = ray_direction_norm[1];
                t = (t + 1.0) * 0.5;

                let color = if hit_info.if_hit {
                    let kd = Vector3d::dot_product(hit_info.normal, ray_direction_norm);
                    Vector3d::new([kd.abs(), kd.abs(), kd.abs()])
                } else {
                    Vector3d::new([1.0, 1.0, 1.0]) * (1.0 - t) + Vector3d::new([0.5, 0.7, 1.0]) * t
                };

                image_writer.set_pixel_color_vec(x, y, &color);

                pixel += viewport_u_delta;
            }
        }

        image_writer.write_to_stdout();
    }
}
