use super::*;

pub mod sphere;

pub trait Drawable {
    fn hit_test(&self, ray: &Ray) -> HitInfo;

    fn fill_info(&self, ray: &Ray, hit_info: &mut HitInfo);
}
