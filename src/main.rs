use std::rc::Rc;


use hit::{Hittable, HitRecord };
use log::{info, Level, debug};

mod color;
mod ray;
mod hit;
mod interval;
mod camera;

use color::Color;
use ray::Ray;
use interval::Interval;

use camera::Camera;

use crate::hit::{HittableList, Sphere};

fn main() {
    env_logger::init();

    let mut world = HittableList::new();
    let sphere_1 = Rc::new(Sphere::new(glam::vec3(0.0, 0.0, -1.0), 0.5));
    let sphere_2 = Rc::new(Sphere::new(glam::vec3(0.0, -100.5, -1.0), 100.0));

    world.add(sphere_1);
    world.add(sphere_2);


    let camera = Camera::new();

    let _ = camera.render(&world);
}
