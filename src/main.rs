mod vec3;
mod ray;
mod hit;
mod sphere;

use std::ops::RangeInclusive;
use std::rc::Rc;

use crate::vec3::{ Point3, Color, Vec3};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hit::{HittableList, Hittable};


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400_u64;
    let image_height = (image_width as f64 / aspect_ratio).floor() as u64;

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    // )));


    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("\r Scanline remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width  - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let color = ray_color(ray, &world);
           
            println!("{}", color);
        }
    }
    eprintln!("\nDone.");
}

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    if let Some(hit) = world.hit(&ray, &RangeInclusive::new(0.0, std::f64::INFINITY)) {
        return Color::new(255.0, 255.0, 255.0) * hit.normal();
    }
    let t = ray.direction().normalize().y();
    Color::new(255.0, 255.0, 255.0) * (1.0 -t) + Color::new(127.0, 178.0, 255.0) * t
}
