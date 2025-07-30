pub mod drawable;

use std::cmp;

use drawable::*;

use crate::{ray::Ray, vector::Vector3d};

pub struct Scene {
    objects: Vec<Box<dyn Drawable>>,
}

pub struct HitInfo {
    pub if_hit: bool,
    pub t: f64,
    pub normal: Vector3d, // NOTE: should be normalized to a unit vector
}

impl Default for HitInfo {
    fn default() -> Self {
        HitInfo {
            if_hit: false,
            t: f64::INFINITY,
            normal: Vector3d::zeros(),
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

        for obj in &self.objects {
            let hit_info = obj.hit_test(ray);
            if hit_info.if_hit {
                res_hit_info.if_hit = true;
                if hit_info.t < res_hit_info.t {
                    res_hit_info.t = hit_info.t;
                    res_hit_info.normal = hit_info.normal;
                }
            }
        }

        res_hit_info
    }
}
