use std::sync::Arc;

use crate::{config, scene::material::Material, utils::interval::Interval, vector::Vector3d};

use super::*;

pub struct Quad {
    corner: Vector3d,
    u: Vector3d,
    v: Vector3d,
    material: Arc<dyn Material>,

    // precomputed helpers
    normal_norm: Vector3d,
    w: Vector3d,
    d: f64,
}

impl Quad {
    pub fn new(corner: Vector3d, u: Vector3d, v: Vector3d, material: Arc<dyn Material>) -> Quad {
        let normal = Vector3d::cross_product(u, v);
        let normal_norm = normal.unit_vec();
        let w = normal / Vector3d::length_squared(normal);
        let d = Vector3d::dot_product(normal_norm, corner);
        Quad {
            corner,
            u,
            v,
            material,
            normal_norm,
            w,
            d,
        }
    }
}

impl Drawable for Quad {
    fn hit_test(&self, ray: &Ray) -> HitInfo {
        let mut res_hit_info = HitInfo::default();

        let t_denominator = Vector3d::dot_product(self.normal_norm, ray.direction);
        if t_denominator.abs() < config::EPS {
            return res_hit_info;
        }

        let t_numerator = self.d - Vector3d::dot_product(self.normal_norm, ray.origin);
        let t = t_numerator / t_denominator;
        if t <= 0.0 {
            return res_hit_info;
        }

        let hit_point = ray.at(t);
        let hit_point_rel = hit_point - self.corner;

        let hit_valid_range = Interval::new(0.0, 1.0);
        let alpha = Vector3d::dot_product(Vector3d::cross_product(hit_point_rel, self.v), self.w);
        let beta = Vector3d::dot_product(Vector3d::cross_product(self.u, hit_point_rel), self.w);
        if hit_valid_range.contains(alpha) && hit_valid_range.contains(beta) {
            res_hit_info.if_hit = true;
            res_hit_info.t = t;
        }

        res_hit_info
    }

    fn scatter(&self, ray: &Ray, hit_info: &mut HitInfo) {
        if Vector3d::dot_product(ray.direction, self.normal_norm) >= 0.0 {
            hit_info.front_face = false;
        } else {
            hit_info.front_face = true;
        };
        hit_info.normal_norm = self.normal_norm;
        hit_info.if_hit = self.material.scatter(ray, hit_info);
    }
}
