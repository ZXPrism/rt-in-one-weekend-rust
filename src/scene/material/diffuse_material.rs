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
    fn scatter(&self, ray_in: &Ray, hit_info: &mut HitInfo) {
        let mut rng = rand::rng();

        let hit_point = ray_in.at(hit_info.t);
        let delta = Vector3d::new([
            rng.random_range(-1.0..=1.0),
            rng.random_range(-1.0..=1.0),
            rng.random_range(-1.0..=1.0),
        ]);

        hit_info.albedo = self.albedo;
        hit_info.scatter_ray = Ray::new(hit_point, hit_info.normal + delta);
    }
}
