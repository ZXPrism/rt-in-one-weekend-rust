pub struct Camera {
    image_width: usize,
    image_height: usize,
    aspect_ratio: f64,
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
}

impl Camera {
    pub fn new(
        image_width: usize,
        viewport_width: f64,
        aspect_ratio: f64,
        focal_length: f64,
    ) -> Self {
        let image_height = ((image_width as f64) / aspect_ratio) as usize;
        Camera {
            image_width,
            image_height,
            aspect_ratio,
            viewport_width,
            viewport_height: ((viewport_width as f64) / (image_height as f64)) as f64,
            focal_length,
        }
    }
}
