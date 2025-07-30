use crate::ImageWriter;
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
            viewport_height: ((viewport_width as f64) / (image_height as f64)) as f64,
            focal_length,
        }
    }

    pub fn render(&self) {
        let mut image_writer = ImageWriter::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let color = Vector3d::new([
                    (x as f64) / (self.image_width as f64),
                    (y as f64) / (self.image_height as f64),
                    0.0,
                ]);
                image_writer.set_pixel_color_vec(x, y, &color);
            }
        }

        image_writer.write_to_stdout();
    }
}
