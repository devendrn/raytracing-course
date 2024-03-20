use crate::vec3::Vec3;

pub struct Ray {
    pub org: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(org: Vec3, dir: Vec3) -> Ray {
        Ray { org, dir }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.org + t * self.dir
    }
}
