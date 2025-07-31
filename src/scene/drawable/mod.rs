use super::*;

pub mod sphere;

pub trait Drawable {
    fn hit_test(&self, ray: &Ray) -> HitInfo;

    /// Note that fill_info will set `if_hit` (which is not set by higher-level callers)
    fn fill_info(&self, ray: &Ray, hit_info: &mut HitInfo);
}
