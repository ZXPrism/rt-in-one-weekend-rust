use super::*;
use rand::Rng;

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
        let delta = Vector3d::random_unit_vector();

        hit_info.albedo = self.albedo;
        hit_info.scatter_ray = Ray::new(hit_point, hit_info.normal + delta);

        // zero vector correction
        const EPS: f64 = 1e-8;
        if hit_info.scatter_ray.direction[0].abs() < EPS
            && hit_info.scatter_ray.direction[1].abs() < EPS
            && hit_info.scatter_ray.direction[2].abs() < EPS
        {
            hit_info.scatter_ray.direction = hit_info.normal;
        }

        true
    }
}
