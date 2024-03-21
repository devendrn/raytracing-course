use std::{fs::File, io::Write};
use crate::rand;
use crate::ray::Ray;
use crate::hittable::{Hittable, HittableList};
use crate::vec3::{clamp, dot, normalize, sqrt, vec3, Vec3};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: u16,
    pub max_depth: u8,
    image_height: i32,
    center: Vec3,
    pixel_delta_uv: Vec3,
    pixel_00_loc: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: u16, max_depth: u8) -> Camera {
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
            samples_per_pixel,
            max_depth
        }
    }

    pub fn render(&self, world: &HittableList) {
        let mut buf = Vec::new();
        writeln!(buf, "P3\n{} {}\n255\n", self.image_width, self.image_height).unwrap();
        for j in 0..self.image_height {
            print!("Rendering: {}%\r", ((j + 1) * 100) / self.image_height);
            for i in 0..self.image_width {
                let mut color = vec3(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color += Self::ray_color(r, self.max_depth, world);
                }
                color = color / self.samples_per_pixel as f32;
                color = Self::linear_to_gamma(color);

                color = clamp(255.9 * color, 0.0, 255.9);

                writeln!(buf, "{} {} {}\n", color.x as u8, color.y as u8, color.z as u8).unwrap();
            }
        }

        let mut render = File::create("out.ppm").expect("Failed to create file!");
        render.write_all(&buf).unwrap();
    }

    fn distort_sample_uv(i: i32, j: i32) -> Vec3 {
        let x = rand::randf32((i + j) as u32);
        vec3(-0.5 + x, -0.5 + (231.23423 * x).fract(), 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_offset = vec3(i as f32, j as f32, 0.0) + Self::distort_sample_uv(i, j);
        let pixel_sample = self.pixel_00_loc + pixel_offset * self.pixel_delta_uv;
        let ray_dir = pixel_sample - self.center;
        Ray::new(self.center, ray_dir)
    }

    fn ray_color(ray: Ray, depth: u8, world: &HittableList) -> Vec3 {
        if depth < 1 {
            return vec3(0.0, 0.0, 0.0);
        }
        
        if let Some(d) = world.hit(&ray, 0.001, f32::MAX) {
            let (ray, attenuation) = d.mat.scatter(&ray, &d);
            if let Some(scatter_ray) = ray {
                return attenuation * Self::ray_color(scatter_ray, depth - 1, world);
            } else {
                return attenuation;
            }
        }

        let unit_dir = normalize(ray.dir);

        let grad = 0.5 + 0.5 * unit_dir.y;
        let mut sky = grad * vec3(0.5, 0.7, 1.0) + (1.0 - grad) * vec3(0.9, 1.0, 1.0);
        sky = sky * (dot(unit_dir, vec3(1.0, 0.4, -0.5)).max(0.0));
        sky
    }

    fn linear_to_gamma(color: Vec3) -> Vec3 {
        sqrt(color)
    }
}
