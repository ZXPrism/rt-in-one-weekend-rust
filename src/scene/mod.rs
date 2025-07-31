pub mod drawable;
pub mod material;

use drawable::*;

use crate::{ray::Ray, vector::Color, vector::Vector3d};

pub struct Scene {
    objects: Vec<Box<dyn Drawable>>,
}

pub struct HitInfo {
    pub if_hit: bool,
    pub t: f64,
    pub normal: Vector3d, // NOTE: should be normalized to a unit vector
    pub scatter_ray: Ray,
    pub albedo: Color,
}

impl Default for HitInfo {
    fn default() -> Self {
        HitInfo {
            if_hit: false,
            t: f64::INFINITY,
            normal: Vector3d::zeros(),
            scatter_ray: Ray::default(),
            albedo: Color::zeros(),
        }
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: vec![] }
    }

    pub fn add_object(&mut self, obj: Box<dyn Drawable>) {
        self.objects.push(obj);
    }

    pub fn hit_test(&self, ray: &Ray) -> HitInfo {
        let mut res_hit_info = HitInfo::default();

        let n = self.objects.len();
        let mut hit_obj_idx = n;
        for i in 0..n {
            let obj = &self.objects[i];

            let hit_info = obj.hit_test(ray);
            if hit_info.if_hit && hit_info.t > 0.00001 {
                if hit_info.t < res_hit_info.t {
                    res_hit_info.t = hit_info.t;
                    hit_obj_idx = i;
                }
            }
        }

        if hit_obj_idx != n {
            res_hit_info.if_hit = true;
            self.objects[hit_obj_idx].fill_info(ray, &mut res_hit_info);
        }

        res_hit_info
    }
}
