use super::*;
use crate::config;

#[derive(Clone)]
pub struct DiffuseMaterial {
    albedo: Color,
}

impl DiffuseMaterial {
    pub fn new(albedo: Color) -> DiffuseMaterial {
        DiffuseMaterial { albedo }
    }
}

impl Material for DiffuseMaterial {
    fn scatter(&self, ray_in: &Ray, hit_info: &mut HitInfo) -> bool {
        let hit_point = ray_in.at(hit_info.t);
        let delta = Vector3d::random_unit_sphere();

        hit_info.albedo = self.albedo;
        hit_info.scatter_ray = Ray::new(hit_point, hit_info.normal_norm + delta);

        // zero vector correction
        if hit_info.scatter_ray.direction[0].abs() < config::EPS
            && hit_info.scatter_ray.direction[1].abs() < config::EPS
            && hit_info.scatter_ray.direction[2].abs() < config::EPS
        {
            hit_info.scatter_ray.direction = hit_info.normal_norm;
        }

        true
    }
}
