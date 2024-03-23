use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use crate::rand;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
   pub x: f32,
   pub y: f32,
   pub z: f32,
}

impl Vec3 {
    fn apply(&self, f: fn(f32) -> f32) -> Vec3 {
        vec3(f(self.x), f(self.y), f(self.z))
    }
}

/* Vec3 operators */

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        vec3(-self.x, -self.y, -self.z)
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        vec3(rhs.x + self, rhs.y + self, rhs.z + self)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Add<f32> for Vec3{
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        rhs + self
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        vec3(rhs.x + self.x, rhs.y + self.y, rhs.z + self.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        vec3(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        vec3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        vec3(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        vec3(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        vec3(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

/* Vec3 Functions */

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn normalize(x: Vec3) -> Vec3 {
    x / length(x)
}

pub fn length(x: Vec3) -> f32 {
    dot(x, x).sqrt()
}

pub fn dot(x1: Vec3, x2: Vec3) -> f32 {
    (x1.x * x2.x) + (x1.y * x2.y) + (x1.z * x2.z)
}

pub fn min(x1: Vec3, x2: Vec3) -> Vec3 {
    vec3(x1.x.min(x2.x), x1.y.min(x2.y), x1.z.min(x2.z))
}

pub fn max(x1: Vec3, x2: Vec3) -> Vec3 {
    vec3(x1.x.max(x2.x), x1.y.max(x2.y), x1.z.max(x2.z))
}

pub fn clamp(x: Vec3, x0: f32, x1: f32) -> Vec3 {
    max(min(x, vec3(x1, x1, x1)), vec3(x0, x0, x0))
}

pub fn sqrt(x: Vec3) -> Vec3 {
    x.apply(|y| y.sqrt()) 
}

pub fn rand_vec3() -> Vec3 {
    let w = rand::randf32(314);
    let (x, y, z) = (w, (w * 14354.0).fract(), (w * 32435.0).fract());
    normalize(vec3(x - 0.5, y - 0.5, z - 0.5))
}

pub fn is_near_zero(x: Vec3) -> bool {
    let s = 1e-8;
    (x.x.abs() < s) && (x.y.abs() < s) && (x.z.abs() < s)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v -  2.0*dot(v, n)*n
}

pub fn refract(v: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(-v, n).min(1.0);
    let r_out_perpendicular = etai_over_etat * (v + cos_theta*n);
    let r_out_parallel = - n * (1.0 - dot(r_out_perpendicular, r_out_perpendicular)).abs().sqrt();
    r_out_perpendicular + r_out_parallel
}

pub fn cross(x1: Vec3, x2: Vec3) -> Vec3 {
    vec3(
        x1.y * x2.z - x1.z * x2.y,
        x1.z * x2.x - x1.x * x2.z,
        x1.z * x2.y - x1.y * x2.x,
    )
}

