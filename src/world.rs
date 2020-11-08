use crate::*;
use rand::thread_rng;

#[derive(Clone)]
pub struct World {
    root: Arc<dyn Hittable + Send + Sync>,
    background: Arc<dyn Texture + Send + Sync>,
}

impl World {
    pub fn new(
        root: Arc<dyn Hittable + Send + Sync>,
        background: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self { root, background }
    }

    pub fn ray_color(&self, ray: Ray, depth: u32) -> Vec3 {
        // Recursive base case
        if depth == 0 {
            return Vec3::zero();
        }

        // We hit something
        if let Some(hit_record) = self.root.hit(&ray, 0.001, std::f64::INFINITY) {
            if let Some((albedo, scatter_ray, pdf)) = hit_record.material.scatter(&ray, &hit_record)
            {
                let mut rng = thread_rng();
                let on_light = Vec3(
                    rng.gen_range(213.0, 343.0),
                    554.0,
                    rng.gen_range(227.0, 332.0),
                );
                let to_light = on_light - hit_record.position;
                let dist_sqd = to_light.length_squared();
                let to_light = to_light.unit_vector();

                if to_light.dot(hit_record.normal) < 0.0 {
                    return hit_record.emitted(&ray);
                }

                let light_area = (343.0 - 213.0) * (332.0 - 227.0);
                // We know the light's normal is exactly on the y axis, so this is fine
                let light_cos = to_light.y().abs();
                if light_cos < 0.000001 {
                    return hit_record.emitted(&ray);
                }

                let pdf = dist_sqd / (light_cos * light_area);
                let scatter_ray = Ray::new(hit_record.position, to_light, ray.time);

                return albedo
                    * hit_record
                        .material
                        .scattering_pdf(&ray, &hit_record, &scatter_ray)
                    * self.ray_color(scatter_ray, depth - 1)
                    / pdf;
            }
            return hit_record.emitted(&ray);
        }

        // Off into infinity
        let (u, v) = get_sphere_uv(&ray.direction);
        self.background.value(u, v, &ray.direction)
    }
}
