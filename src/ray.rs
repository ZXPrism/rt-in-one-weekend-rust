use crate::vector::*;

pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3f {
        self.origin + self.direction * t
    }
}
