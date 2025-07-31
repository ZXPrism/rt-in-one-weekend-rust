use super::*;

pub struct MetalMaterial {
    albedo: Color,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> MetalMaterial {
        MetalMaterial { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, hit_info: &mut HitInfo) -> bool {
        let hit_point = ray_in.at(hit_info.t);

        hit_info.albedo = self.albedo;
        hit_info.scatter_ray.origin = hit_point;

        let mut reflected = Vector3d::reflect(ray_in.direction, hit_info.normal);
        reflected = reflected.unit_vec() + Vector3d::random_unit_vector() * self.fuzz;

        if Vector3d::dot_product(reflected, hit_info.normal) <= 0.0 {
            return false;
        }

        hit_info.scatter_ray.direction = reflected;

        true
    }
}
