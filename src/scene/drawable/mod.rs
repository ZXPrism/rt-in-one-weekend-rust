use super::*;

pub mod cuboid;
pub mod sphere;

pub trait Drawable: Send + Sync {
    fn hit_test(&self, ray: &Ray) -> HitInfo;

    /// Note that scatter will set `if_hit` (which is not set by higher-level callers)
    fn scatter(&self, ray: &Ray, hit_info: &mut HitInfo);
}
