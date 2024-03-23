use std::{fs::File, io::Write};
use crate::rand;
use crate::ray::Ray;
use crate::hittable::{Hittable, HittableList};
use crate::vec3::{clamp, cross, dot, length, normalize, sqrt, vec3, Vec3};

pub struct Camera {
    pub samples_per_pixel: u16,
    pub max_depth: u8,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub v_up: Vec3,
    image_height: i32,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_00_loc: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let vec3_0 = vec3(0.0, 0.0, 0.0);
        Camera {
            image_width: 256,
            image_height: 256,
            samples_per_pixel: 32,
            max_depth: 8,
            aspect_ratio: 1.0,
            fov: 90.0,
            pixel_delta_u: vec3_0,
            pixel_delta_v: vec3_0,
            pixel_00_loc: vec3_0,
            look_from: vec3_0,
            look_at: vec3_0,
            v_up: vec3_0,
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;

        let focal_len = length(self.look_from - self.look_at);
        let theta = self.fov.to_radians();
        let h = (0.5 * theta).tan();

        let viewport_height = 2.0 * h * focal_len;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let w = normalize(self.look_from - self.look_at);
        let u = normalize(cross(self.v_up, w));
        let v = cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = - viewport_height * v;

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        let viewport_top_left = self.look_from - (focal_len * w) - 0.5 * (viewport_u + viewport_v);

        self.pixel_00_loc = viewport_top_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&mut self, world: &HittableList) {
        Self::initialize(self);
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
        let pixel_sample = 
            self.pixel_00_loc +
            (pixel_offset.x * self.pixel_delta_u) +
            (pixel_offset.y * self.pixel_delta_v);
       let ray_dir = pixel_sample - self.look_from;
        Ray::new(self.look_from, ray_dir)
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
