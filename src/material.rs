use crate::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn from_albedo(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        let scatter_ray = Ray::new(hit_record.position, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scatter_ray))
    }
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn from_albedo(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray_in.direction.reflect(hit_record.normal);
        let scatter_ray = Ray::new(hit_record.position, reflected);
        let attenuation = self.albedo;
        if scatter_ray.direction.dot(hit_record.normal) > 0.0 {
            Some((attenuation, scatter_ray))
        } else {
            None
        }
    }
}
