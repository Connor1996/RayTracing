mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

use std::ops::RangeInclusive;
use std::rc::Rc;

use crate::camera::{Camera, APSECT_RATIO};
use crate::hit::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3};

use rand::Rng;

const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

fn main() {
    // Image
    let image_width = 640_u64;
    let image_height = (image_width as f64 / APSECT_RATIO).floor() as u64;

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(
        // ground
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));
    world.add(Rc::new(Sphere::new(
        // center
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
    )));
    world.add(Rc::new(Sphere::new(
        // left
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::new(
        // right
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3)),
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
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                let sample_color = ray_color(ray, &world, MAX_DEPTH);
                color += sample_color;
            }

            println!("{}", color / SAMPLES_PER_PIXEL as f64);
        }
    }
    eprintln!("\nDone.");
}

fn ray_color(ray: Ray, world: &HittableList, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(&ray, &RangeInclusive::new(0.001, std::f64::INFINITY)) {
        if let Some((attenuation, scattered_ray)) = hit.material.scatter(&ray, &hit) {
            return attenuation * ray_color(scattered_ray, &world, depth - 1) * 0.5;
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let t = (ray.direction().normalize().y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
