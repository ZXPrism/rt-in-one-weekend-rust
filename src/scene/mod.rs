pub mod drawable;

use drawable::*;

pub struct Scene {
    objects: Vec<Box<dyn Drawable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: vec![] }
    }

    pub fn add_object(&mut self, obj: Box<dyn Drawable>) {
        self.objects.push(obj);
    }

    pub fn hit(&self) {}
}
