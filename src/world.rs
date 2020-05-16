use crate::*;

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
            if let Some((attenuation, scatter_ray)) = hit_record.material.scatter(&ray, &hit_record)
            {
                return attenuation * self.ray_color(scatter_ray, depth - 1);
            }
            return hit_record.emitted();
        }

        // Off into infinity
        let (u, v) = get_sphere_uv(&ray.direction);
        self.background.value(u, v, &ray.direction)
    }
}