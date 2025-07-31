use std::rc::Rc;

use crate::{scene::material::Material, vector::Vector3d};

use super::*;

pub struct Sphere {
    center: Vector3d,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3d, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Drawable for Sphere {
    fn hit_test(&self, ray: &Ray) -> HitInfo {
        let mut res_hit_info = HitInfo::default();

        let origin_vec = self.center - ray.origin;
        let a = Vector3d::dot_product(ray.direction, ray.direction);
        let b = -2.0 * Vector3d::dot_product(ray.direction, origin_vec);
        let c = Vector3d::dot_product(origin_vec, origin_vec) - self.radius * self.radius;

        let det = b * b - 4.0 * a * c;
        if det >= 0.0 {
            let det_sqrt = det.sqrt();
            let t1 = (-b - det_sqrt) / (2.0 * a);
            let t2 = (-b + det_sqrt) / (2.0 * a);

            if t1 >= 0.0 {
                res_hit_info.if_hit = true;
                res_hit_info.t = t1;
            } else if t2 >= 0.0 {
                res_hit_info.if_hit = true;
                res_hit_info.t = t2;
            }
        }

        res_hit_info
    }

    fn scatter(&self, ray: &Ray, hit_info: &mut HitInfo) {
        hit_info.normal = ray.at(hit_info.t) - self.center;
        hit_info.normal /= self.radius;
        hit_info.front_face = Vector3d::dot_product(ray.direction, hit_info.normal) <= 0.0;
        hit_info.if_hit = self.material.scatter(ray, hit_info);
    }
}
