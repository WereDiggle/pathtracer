use rand::random;

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
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
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

pub struct Dielectric {
    pub refract_index: f64,
}

impl Dielectric {
    pub fn new(ref_index: f64) -> Self {
        Self {
            refract_index: ref_index,
        }
    }

    fn schlick(&self, cosine: f64) -> f64 {
        let r0 = (1.0 - self.refract_index) / (1.0 + self.refract_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3(1.0, 1.0, 1.0);
        let etai_over_etat = if hit_record.front_face {
            1.0 / self.refract_index
        } else {
            self.refract_index
        };

        let cos_theta = hit_record.normal.dot(-ray_in.direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob = self.schlick(cos_theta);
        let scatter_direction =
            if etai_over_etat * sin_theta > 1.0 || random::<f64>() < reflect_prob {
                ray_in.direction.reflect(hit_record.normal)
            } else {
                ray_in.direction.refract(hit_record.normal, etai_over_etat)
            };

        Some((
            attenuation,
            Ray::new(hit_record.position, scatter_direction),
        ))
    }
}
