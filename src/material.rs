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
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clam(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray_in.direction.reflect(hit_record.normal);
        let scatter_ray = Ray::new(
            hit_record.position,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        if scatter_ray.direction.dot(hit_record.normal) > 0.0 {
            Some((attenuation, scatter_ray))
        } else {
            None
        }
    }
}
