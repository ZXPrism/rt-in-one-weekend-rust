use super::*;

pub struct MetalMaterial {
    albedo: Color,
}

impl MetalMaterial {
    pub fn new(albedo: Color) -> MetalMaterial {
        MetalMaterial { albedo }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, hit_info: &mut HitInfo) {
        let hit_point = ray_in.at(hit_info.t);

        hit_info.albedo = self.albedo;
        hit_info.scatter_ray.origin = hit_point;
        hit_info.scatter_ray.direction = ray_in.direction
            - hit_info.normal * 2.0 * Vector3d::dot_product(ray_in.direction, hit_info.normal);
    }
}
