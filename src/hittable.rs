use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Vec3, dot};

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Vec3, outward_normal: Vec3, t: f32, mat: &'a dyn Material) -> HitRecord<'a> {
        HitRecord { p, t, normal: outward_normal, front_face: false, mat }
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

pub struct HittableList<'a> {
    pub objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object)
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        for item in self.objects.iter() {
            let hit = item.hit(ray, ray_tmin, ray_tmax);
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
