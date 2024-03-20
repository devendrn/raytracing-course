mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;

use vec3::vec3;
use camera::Camera;
use hittable::{Hittables, HittableList};
use sphere::Sphere;

fn main() {
    let mut world = HittableList::new();

    // main sphere
    world.add(Hittables::Sphere(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5)));

    // floor
    world.add(Hittables::Sphere(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0)));
    
    let cam = Camera::new(16.0 / 9.0, 400);
    cam.render(&world);
}

