use crate::{scene::material::Material, utils::interval::Interval, vector::Vector3d};

use super::*;

pub struct Cuboid {
    center: Vector3d,
    slab_x: Interval,
    slab_y: Interval,
    slab_z: Interval,
    material: Box<dyn Material>,
}

impl Cuboid {
    pub fn new(
        center: Vector3d,
        slab_x: Interval,
        slab_y: Interval,
        slab_z: Interval,
        radius: f64,
        material: Box<dyn Material>,
    ) -> Cuboid {
        Cuboid {
            center,
            slab_x,
            slab_y,
            slab_z,
            material,
        }
    }
}

impl Drawable for Cuboid {
    fn hit_test(&self, ray: &Ray) -> HitInfo {
        let mut res_hit_info = HitInfo::default();

        // check if hit
        // if hit, the ray will "pierce in" the cuboid
        // the projection of the ray on 3 axes must all overlap with the slab interval

        res_hit_info
    }

    fn scatter(&self, ray: &Ray, hit_info: &mut HitInfo) {
        // set normal
        // ...
        hit_info.front_face = Vector3d::dot_product(ray.direction, hit_info.normal) <= 0.0;
        hit_info.if_hit = self.material.scatter(ray, hit_info);
    }
}
