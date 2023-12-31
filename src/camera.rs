
use std::io::{self, Write};
use glam::Vec3;

use log::{info, Level, debug};

use crate::hit::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Camera {

}

impl Camera {
    pub fn new() -> Self {
        Camera {

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

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()>  {
        // image
        let aspect_ratio = 16.0 / 9.0 as f32; 
        let img_width = 400;
        let img_height = (img_width as f32 / aspect_ratio) as i32;

        // camera
        let viewport_height: f32 = 2.0;
        let viewport_width = viewport_height * (img_width as f32 / img_height as f32);
        let focal_length = 1.0;
        let camera_center = glam::vec3(0.0,0.0,0.0);

        // calculate the vectors across the horizontal
        let viewport_u = glam::vec3(viewport_width, 0.0, 0.0);
        let viewport_v = glam::vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / img_width as f32; 
        let pixel_delta_v = viewport_v / img_height as f32;

        // calculate the location of the upper left pixel
        let viewport_upper_left = camera_center 
            - glam::vec3(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let ppm_header = format!("P3\n {} {} \n255\n", img_width, img_height);
        io::stdout().write_all(ppm_header.as_bytes())?;

        let mut ppm_data: String = "".to_owned();
        for j in 0..img_height {
            info!("Scanlines remaining: {}", img_height - j);

            for i in 0..img_width {
                let pixel_center: Vec3 = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32  * pixel_delta_v);
                let ray_direction = pixel_center - camera_center;
                let r = Ray::new(camera_center, ray_direction);
                

                let pixel_color = self.ray_color(&r, world);
                // let pixel_color_str = format!("{} {} {}", pixel_color.x, pixel_color.y, pixel_color.z);
                let pixel_color_str = format!("{} {} {}\n",
                    (255.999 * pixel_color.x) as i32,
                    (255.999 * pixel_color.y) as i32,
                    (255.999 * pixel_color.z) as i32);
                
                ppm_data.push_str(&pixel_color_str);
            }
        }

        io::stdout().write_all(ppm_data.as_bytes())?;

        info!("Finished rendering.");

        Ok(())
    }
}