use crate::vector::*;

pub struct Ray {
    pub origin: Vector3d,
    pub direction: Vector3d,
}

impl Ray {
    pub fn new(origin: Vector3d, direction: Vector3d) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3d {
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Vector3d::zeros(),
            direction: Vector3d::zeros(),
        }
    }
}
