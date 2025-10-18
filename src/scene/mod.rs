pub mod drawable;
pub mod material;

use std::usize;

use drawable::*;

use crate::{
    config,
    ray::Ray,
    vector::{Color, Vector3d},
};

pub struct Scene {
    objects: Vec<Box<dyn Drawable>>,
}

pub struct HitInfo {
    pub if_hit: bool,
    pub front_face: bool,
    pub t: f64,
    pub normal_norm: Vector3d,
    pub scatter_ray: Ray,
    pub albedo: Color,
    pub aux: usize,
}

impl Default for HitInfo {
    fn default() -> Self {
        HitInfo {
            if_hit: false,
            front_face: true,
            t: f64::INFINITY,
            normal_norm: Vector3d::zeros(),
            scatter_ray: Ray::default(),
            albedo: Color::zeros(),
            aux: usize::MAX, // for debug
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
            if hit_info.if_hit && hit_info.t > config::EPS {
                if hit_info.t < res_hit_info.t {
                    res_hit_info.t = hit_info.t;
                    res_hit_info.aux = hit_info.aux;
                    hit_obj_idx = i;
                }
            }
        }

        if hit_obj_idx != n {
            self.objects[hit_obj_idx].scatter(ray, &mut res_hit_info);
        }

        res_hit_info
    }
}
