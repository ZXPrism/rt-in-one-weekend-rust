use rand::Rng;

use super::*;

pub struct DielectricMaterial {
    refraction_index: f64,
}

impl DielectricMaterial {
    pub fn new(refraction_index: f64) -> DielectricMaterial {
        DielectricMaterial { refraction_index }
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, hit_info: &mut HitInfo) -> bool {
        let hit_point = ray_in.at(hit_info.t);

        let ray_in_norm = ray_in.direction.unit_vec();

        let co_norm = if hit_info.front_face {
            hit_info.normal * -1.0
        } else {
            hit_info.normal
        }; // the norm which is on the same side of the surface with the incident ray (i.e. dot product >= 0)

        let ri = if hit_info.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = Vector3d::dot_product(ray_in_norm, co_norm);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Schlick Approximation
        let mut rng = rand::rng();
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 = r0 * r0;

        if ri * sin_theta >= 1.0
            || r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5) > rng.random_range(0.0..=1.0)
        {
            hit_info.scatter_ray.direction = Vector3d::reflect(ray_in.direction, hit_info.normal);
        } else {
            let scatter_ray_perp =
                (ray_in_norm - co_norm * Vector3d::dot_product(ray_in_norm, co_norm)) * ri;
            let scatter_ray_para = co_norm * (1.0 - scatter_ray_perp.length_squared()).sqrt();
            hit_info.scatter_ray.direction = scatter_ray_perp + scatter_ray_para;
        }

        hit_info.albedo = Color::new([1.0, 1.0, 1.0]);
        hit_info.scatter_ray.origin = hit_point;

        true
    }
}
