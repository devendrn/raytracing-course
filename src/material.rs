use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{is_near_zero, rand_vec3, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
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
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit.normal + rand_vec3();
        if is_near_zero(scatter_direction) {
            scatter_direction = hit.normal;
        }
        Some((Ray::new(hit.p, scatter_direction), self.albedo))
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
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let scatter_direction = hit.normal + self.roughness*rand_vec3();
        Some((Ray::new(hit.p, scatter_direction), self.albedo))
    }
}
