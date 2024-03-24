mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod rand;
mod material;

use material::{Dielectric, Lambertian, Metal};
use vec3::vec3;
use camera::Camera;
use hittable::HittableList;
use sphere::Sphere;

fn main() {
    let material_obj1 = Lambertian::new(vec3(1.0, 1.0, 1.0));
    let material_obj2 = Metal::new(vec3(0.8, 0.8, 0.8) * 0.8, 0.03);
    let material_obj3 = Dielectric::new(vec3(1.0, 1.0, 1.0), 0.03, 1.4);
    let material_ground = Lambertian::new(vec3(0.0, 1.0, 0.0));

    let objects = vec![
        Sphere::new(vec3(0.0, 0.0, -1.0), 0.5, &material_obj1),
        Sphere::new(vec3(1.0, 0.0, -1.0), 0.5, &material_obj3),
        Sphere::new(vec3(-1.0, 0.0, -1.0), 0.5, &material_obj2),
        Sphere::new(vec3(0.0, -100.5, -1.0), 100.0, &material_ground),
    ];

    let mut world = HittableList::new();
    for obj in objects.iter() {
        world.add(obj);
    }
    
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 360;
    cam.samples_per_pixel = 32;
    cam.max_depth = 18;
    cam.fov = 20.0;
    cam.look_from = vec3(-2.0, 2.0, 1.0);
    cam.look_at = vec3(0.0, 0.0, -1.0);
    cam.v_up = vec3(0.0, 1.0, 0.0);
    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}
