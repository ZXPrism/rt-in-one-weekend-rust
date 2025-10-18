use std::sync::Arc;

use crate::{
    scene::{drawable::quad::Quad, material::Material},
    vector::Vector3d,
};

use super::*;

pub struct Parallelepiped {
    face_array: [Quad; 6],
}

impl Parallelepiped {
    pub fn new(
        corner: Vector3d,
        u: Vector3d,
        v: Vector3d,
        w: Vector3d,
        material: Arc<dyn Material>,
    ) -> Parallelepiped {
        let corner_opposite = corner + u + v + w;

        let make_quad = |curr_corner: Vector3d, a: Vector3d, b: Vector3d, c: Vector3d| -> Quad {
            let normal = Vector3d::cross_product(a, b);
            if Vector3d::dot_product(normal, c) > 0.0 {
                Quad::new(curr_corner, b, a, material.clone())
            } else {
                Quad::new(curr_corner, a, b, material.clone())
            }
        };

        let face_array = [
            make_quad(corner, u, v, w),
            make_quad(corner, u, w, v),
            make_quad(corner, v, w, u),
            make_quad(corner_opposite, -u, -v, -w),
            make_quad(corner_opposite, -u, -w, -v),
            make_quad(corner_opposite, -v, -w, -u),
        ];

        Parallelepiped { face_array }
    }
}

impl Drawable for Parallelepiped {
    fn hit_test(&self, ray: &Ray) -> HitInfo {
        let mut res_hit_info = HitInfo::default();

        for (index, face) in self.face_array.iter().enumerate() {
            let hit_info = face.hit_test(ray);
            if hit_info.if_hit && hit_info.t < res_hit_info.t {
                res_hit_info.if_hit = true;
                res_hit_info.t = hit_info.t;
                res_hit_info.aux = index;
            }
        }

        res_hit_info
    }

    fn scatter(&self, ray: &Ray, hit_info: &mut HitInfo) {
        let face = &self.face_array[hit_info.aux];
        face.scatter(ray, hit_info);
    }
}
