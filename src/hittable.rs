use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Vec3, dot};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, outward_normal: Vec3, t: f32) -> Self {
        HitRecord { p, t, normal: outward_normal, front_face: false }
    }

    pub fn set_face_normal(&mut self, ray: &Ray) {
        self.front_face = dot(ray.dir, self.normal) < 0.0;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Hittables>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Hittables) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        for item in self.objects.iter() {
            let hit = match item {
                Hittables::Sphere(s) => s.hit(ray, ray_tmin, ray_tmax),
            };

            let Some(s) = hit else { continue };
            if let Some(ref c) = closest_hit {
                if s.t < c.t {
                    closest_hit = Some(s);
                }
            } else {
                closest_hit = Some(s);
            }
        };

        closest_hit
    }
}

pub enum Hittables {
    Sphere(Sphere),
}
