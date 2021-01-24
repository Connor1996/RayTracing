mod vec3;
mod ray;
mod hit;
mod sphere;
mod camera;
mod util;

use std::ops::RangeInclusive;
use std::rc::Rc;

use crate::vec3::{ Point3, Color };
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hit::{HittableList, Hittable};
use crate::camera::{Camera, APSECT_RATIO};

use rand::Rng;

const SAMPLES_PER_PIXEL : usize = 100;

fn main() {
    // Image
    let image_width = 400_u64;
    let image_height = (image_width as f64 / APSECT_RATIO).floor() as u64;

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let camera = Camera::new();
    let mut rng = rand::thread_rng();
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("\r Scanline remaining: {} ", j);
        for i in 0..image_width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width  - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                let sample_color = ray_color(ray, &world);
                color += sample_color;
            }
           
            println!("{}", color / SAMPLES_PER_PIXEL as f64);
        }
    }
    eprintln!("\nDone.");
}

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    if let Some(hit) = world.hit(&ray, &RangeInclusive::new(0.0, std::f64::INFINITY)) {
        let normal = hit.normal();
        return Color::new(255.0, 255.0, 255.0) * normal.normalize();
    }
    let t = ray.direction().normalize().y();
    Color::new(255.0, 255.0, 255.0) * (1.0 -t) + Color::new(127.0, 178.0, 255.0) * t
}
