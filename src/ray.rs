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

    pub fn get_intersection_xz_plane(&self, y: f64) -> (f64, bool) {
        // ;
    }

    pub fn get_intersection_xy_plane(&self, z: f64) -> (f64, bool) {
        // ;
    }

    pub fn get_intersection_yz_plane(&self, x: f64) -> (f64, bool) {
        // ;
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
