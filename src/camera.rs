use std::{fs::File, io::Write};
use crate::ray::Ray;
use crate::hittable::{Hittable, HittableList};
use crate::vec3::{Vec3, vec3, clamp, normalize};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    center: Vec3,
    pixel_delta_uv: Vec3,
    pixel_00_loc: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32) -> Camera {
        let image_height = (image_width as f32 / aspect_ratio) as i32;

        let focal_len = 1.0;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let center = vec3(0.0, 0.0, 0.0);

        let viewport_uv = vec3(viewport_width, -viewport_height, 0.0);

        let pixel_delta_uv = viewport_uv / vec3(image_width as f32, image_height as f32, 1.0);

        let viewport_top_left = center - vec3(0.0, 0.0, focal_len) - 0.5 * viewport_uv;

        let pixel_00_loc = viewport_top_left + 0.5 * pixel_delta_uv;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel_delta_uv,
            pixel_00_loc,
        }
    }

    pub fn render(&self, world: &HittableList) {
        let mut buf = Vec::new();
        writeln!(buf, "P3\n{} {}\n255\n", self.image_width, self.image_height).unwrap();
        for j in 0..self.image_height {
            print!("Rendering: {j}\r");
            for i in 0..self.image_width {
                let pixel_center = self.pixel_00_loc + vec3(i as f32, j as f32, 0.0) * self.pixel_delta_uv;
                let ray_dir = pixel_center - self.center;

                let r = Ray::new(self.center, ray_dir);
                let color = clamp(255.9 * Self::ray_color(r, world), 0.0, 255.9);

                writeln!(buf, "{} {} {}\n", color.x as u8, color.y as u8, color.z as u8).unwrap();
            }
        }
        println!("Rendering: COMPLETE!");

        let mut render = File::create("out.ppm").expect("Failed to create file!");
        render.write_all(&buf).unwrap();
    }

    fn ray_color(ray: Ray, world: &HittableList) -> Vec3 {
        if let Some(d) = world.hit(&ray, 0.0, f32::MAX) {
            return 0.5 + d.normal * 0.5;
        }

        let unit_dir = normalize(ray.dir);

        let grad = 0.5 + 0.5 * unit_dir.y;
        grad * vec3(0.5, 0.7, 1.0) + (1.0 - grad) * vec3(0.9, 1.0, 1.0)
    }
}
