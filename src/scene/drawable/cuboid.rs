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

        // as long as we intersect with one of the slab plane, the other plane must also intersects
        let (tx1, tx1_intersect) = ray.get_intersection_yz_plane(self.slab_x.left);
        let (tx2, _) = ray.get_intersection_yz_plane(self.slab_x.right);
        let interval_x = if tx1_intersect {
            Interval::new(ray.at(tx1)[0], ray.at(tx2)[0])
        } else {
            Interval::new(ray.origin[0], ray.origin[0])
        };
        if !interval_x.overlap(self.slab_x) {
            return res_hit_info;
        }

        let (ty1, ty1_intersect) = ray.get_intersection_xz_plane(self.slab_y.left);
        let (ty2, _) = ray.get_intersection_xz_plane(self.slab_y.right);
        let interval_y = if ty1_intersect {
            Interval::new(ray.at(ty1)[1], ray.at(ty2)[1])
        } else {
            Interval::new(ray.origin[1], ray.origin[1])
        };
        if !interval_y.overlap(self.slab_y) {
            return res_hit_info;
        }

        let (tz1, tz1_intersect) = ray.get_intersection_xy_plane(self.slab_z.left);
        let (tz2, _) = ray.get_intersection_xy_plane(self.slab_z.right);
        let interval_z = if tz1_intersect {
            Interval::new(ray.at(tz1)[2], ray.at(tz2)[2])
        } else {
            Interval::new(ray.origin[2], ray.origin[2])
        };
        if !interval_z.overlap(self.slab_z) {
            return res_hit_info;
        }

        res_hit_info.if_hit = true;
        res_hit_info.t = 1.0; // SET THIS!

        res_hit_info
    }

    fn scatter(&self, ray: &Ray, hit_info: &mut HitInfo) {
        // set normal
        // ...
        hit_info.front_face = Vector3d::dot_product(ray.direction, hit_info.normal) <= 0.0;
        hit_info.if_hit = self.material.scatter(ray, hit_info);
    }
}
