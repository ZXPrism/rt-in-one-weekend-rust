use super::*;

pub mod sphere;

pub trait Drawable {
    fn hit_test(&self, ray: &Ray) -> HitInfo;
}
