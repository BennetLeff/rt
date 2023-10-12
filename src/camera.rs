
use std::io::{self, Write};
use rand::prelude::*;

use glam::Vec3;

use log::{info, Level, debug};

use crate::hit::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Camera {
    samples_per_pixel: u8,
    image_width: i32,
    image_height: i32,
    pixel_delta_u: glam::Vec3,
    pixel_delta_v: glam::Vec3,
    viewport_upper_left: glam::Vec3,
    center: glam::Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let samples_per_pixel = 10;
        let image_width = 400;
        let aspect_ratio = 16.0 / 9.0;

        let image_height = (image_width as f32 / aspect_ratio) as i32;

        // Camera parameters
        let viewport_height: f32 = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let focal_length = 1.0;
        let camera_center = glam::Vec3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and vertical dimensions
        let viewport_u = glam::Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = glam::Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = camera_center
            - glam::Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        Camera {
            samples_per_pixel,
            image_width,
            image_height,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            center: camera_center,
        }
    }

    pub fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> glam::Vec3 {
        let mut rec = HitRecord::default();
        let base_interval = Interval::with_values(0.0, f32::INFINITY);

        if world.hit(ray, base_interval, &mut rec) {
            0.5 * (rec.normal + glam::vec3(1.0, 1.0, 1.0))
        } else {
            let unit_direction = ray.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a)*glam::vec3(1.0, 1.0, 1.0) + a * glam::vec3(0.5, 0.5, 0.5)
        }
    }

    fn pixel_color_to_str(&self, pixel_color: glam::Vec3, samples_per_pixel: u8) -> String {
        let scale = 1.0 / samples_per_pixel as f32;
        
        let r = pixel_color.x * scale;
        let g = pixel_color.y * scale;
        let b = pixel_color.z * scale;
                
        format!("{} {} {}\n",
            (256.0 * r.clamp(0.0, 0.999)) as i32,
            (256.0 * g.clamp(0.0, 0.999)) as i32,
            (256.0 * b.clamp(0.0, 0.999)) as i32)
    }

    fn pixel_sample_square(&self) -> glam::Vec3 {
        let mut rng = rand::thread_rng();
        let px: f32 = -0.5 * rng.gen::<f32>();
        let py: f32 = -0.5 * rng.gen::<f32>();
        px * self.pixel_delta_u + py*self.pixel_delta_v
    }

    fn get_ray(&self, pixel00_loc: glam::Vec3, center: glam::Vec3, i: i32, j: i32) -> Ray {
        let pixel_center = pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        
        let ray_origin = center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()>  {

        let pixel00_loc = self.viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let ppm_header = format!("P3\n {} {} \n255\n", self.image_width, self.image_height);
        io::stdout().write_all(ppm_header.as_bytes())?;

        let mut ppm_data: String = "".to_owned();
        for j in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - j);

            for i in 0..self.image_width {
                let mut accumulated_pixel_color = glam::vec3(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(pixel00_loc, self.center, i, j);

                    accumulated_pixel_color += self.ray_color(&ray, world)
                }
                

                let pixel_color_str = self.pixel_color_to_str(accumulated_pixel_color, self.samples_per_pixel);
                
                ppm_data.push_str(&pixel_color_str);
            }
        }

        io::stdout().write_all(ppm_data.as_bytes())?;

        info!("Finished rendering.");

        Ok(())
    }
}