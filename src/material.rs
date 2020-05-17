use std::sync::Arc;

use rand::random;

use crate::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color3 {
        Vec3::zero()
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn from_texture(albedo: Arc<dyn Texture + Send + Sync>) -> Arc<Self> {
        Arc::new(Self { albedo })
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Arc<Self> {
        Arc::new(Self {
            albedo: SolidColor::new(r, g, b),
        })
    }

    pub fn from_color3(color: Color3) -> Arc<Self> {
        Arc::new(Self {
            albedo: Arc::new(SolidColor(color)),
        })
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        let scatter_ray = Ray::new(hit.position, scatter_direction, ray_in.time);
        let attenuation = self.albedo.value(hit.u, hit.v, &hit.position);
        Some((attenuation, scatter_ray))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Arc<Self> {
        Arc::new(Self {
            albedo,
            fuzz: fuzz.clam(0.0, 1.0),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray_in.direction.reflect(hit_record.normal);
        let scatter_ray = Ray::new(
            hit_record.position,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray_in.time,
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
    pub fn new(ref_index: f64) -> Arc<Self> {
        Arc::new(Self {
            refract_index: ref_index,
        })
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
            Ray::new(hit_record.position, scatter_direction, ray_in.time),
        ))
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn from_texture(emit: Arc<dyn Texture + Send + Sync>) -> Arc<Self> {
        Arc::new(Self { emit })
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color3 {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    pub fn from_texture(albedo: Arc<dyn Texture + Send + Sync>) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let scattered = Ray::new(
            hit_record.position,
            Vec3::random_in_unit_sphere(),
            ray_in.time,
        );
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.position);
        Some((attenuation, scattered))
    }
}
