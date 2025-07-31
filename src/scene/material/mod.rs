pub mod dielectric_material;
pub mod diffuse_material;
pub mod metal_material;

use crate::vector::Color;

use super::*;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_info: &mut HitInfo) -> bool;
}
