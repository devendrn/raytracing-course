use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::vec3::{dot, Vec3};
use crate::material::Material;

pub struct Sphere<'a> {
    radius: f32,
    center: Vec3,
    mat: &'a dyn Material,
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let oc = ray.org - self.center;
        let a = dot(ray.dir, ray.dir);
        let half_b = dot(oc, ray.dir);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut t = (-half_b - sqrtd) / a;
        if t <= ray_tmin || t >= ray_tmax {
            t = (-half_b + sqrtd) / a;
            if t <= ray_tmin || t >= ray_tmax {
                return None;
            }
        }

        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let mut hit = HitRecord::new(p, outward_normal, t, self.mat);
        hit.set_face_normal(ray);
        Some(hit)
    }
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f32, mat: &'a dyn Material) -> Sphere {
        Sphere { center, radius, mat }
    }
}
