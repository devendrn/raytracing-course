use crate::hittable::HitRecord;
use crate::rand::randf32;
use crate::ray::Ray;
use crate::vec3::{dot, is_near_zero, normalize, rand_vec3, reflect, refract, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Option<Ray>, Vec3);
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> (Option<Ray>, Vec3) {
        let mut scatter_direction = hit.normal + rand_vec3();
        if is_near_zero(scatter_direction) {
            scatter_direction = hit.normal;
        }
        (Some(Ray::new(hit.p, scatter_direction)), self.albedo)
    }
}

pub struct Metal {
    albedo: Vec3,
    roughness: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, roughness: f32) -> Self {
        Metal { albedo, roughness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Option<Ray>, Vec3) {
        let reflected = reflect(normalize(ray.dir), hit.normal);
        let scatterred = reflected + self.roughness*rand_vec3();

        (Some(Ray::new(hit.p, scatterred)), self.albedo)
    }
}

pub struct Dielectric {
    albedo: Vec3,
    roughness: f32,
    ior: f32,
}

impl Dielectric {
    pub fn new(albedo: Vec3, roughness: f32, ior: f32) -> Self {
        Dielectric { albedo, roughness, ior }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0)*((1.0 - cosine).powf(5.0))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> (Option<Ray>, Vec3) {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let normalized_dir = normalize(ray.dir);
        let cos_theta = dot(-normalized_dir, hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = { 
            refraction_ratio * sin_theta > 1.0 ||
            Self::reflectance(cos_theta, refraction_ratio) > randf32(1)
        };

        let direction = if cannot_refract {
            reflect(normalize(ray.dir), hit.normal)
        } else {
            refract(normalize(ray.dir), hit.normal, refraction_ratio)
        } + self.roughness * rand_vec3();

        (Some(Ray::new(hit.p, direction)), self.albedo)
    }
}
