mod aabb;
mod bvh;
mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod texture;
mod util;
mod vec3;

use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};

use crate::camera::{Camera, APSECT_RATIO};
use crate::hit::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::{MovingSphere, Sphere};
use crate::texture::{Checker, SolidColor};
use crate::util::{random_f64, random_f64_range};
use crate::vec3::{Color, Point3, Vec3};

use crossbeam::channel::unbounded;
use rand::Rng;

const SAMPLES_PER_PIXEL: usize = 500;
const MAX_DEPTH: usize = 50;

const MAX_THREADS: usize = 12;

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Checker::new(
        Box::new(SolidColor::new(0.2, 0.3, 0.1)),
        Box::new(SolidColor::new(0.9, 0.9, 0.9)),
    );
    let ground_material = Arc::new(Lambertian::new(Box::new(checker)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.0 {
                if choose_mat < 0.8 {
                    // diffuse
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(Box::new(SolidColor::new(
                            random_f64(),
                            random_f64(),
                            random_f64(),
                        )))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, random_f64_range(0.0..0.5), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Metal::new(
                            Color::new(
                                random_f64_range(0.5..1.0),
                                random_f64_range(0.5..1.0),
                                random_f64_range(0.5..1.0),
                            ),
                            random_f64_range(0.0..0.5) / 2.0,
                        )),
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Box::new(SolidColor::new(0.4, 0.2, 0.1)))),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() {
    // Image
    let image_width = 400_usize;
    let image_height = (image_width as f64 / APSECT_RATIO).floor() as usize;

    // World
    let world = random_scene();

    // Camera
    let camera = Arc::new(Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        20.0,
        0.1,  // aperture
        10.0, // dist_to_focus
        0.0,
        1.0,
    ));
    println!("P3\n{} {}\n255", image_width, image_height);

    let (s, r) = unbounded();
    let canvas = Arc::new(Mutex::new(vec![
        vec![Color::default(); image_width];
        image_height
    ]));
    let finished = Arc::new(Mutex::new(vec![0; image_height]));

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            s.send((i, j)).unwrap();
        }
    }

    let world = Arc::new(world.into_bvh());
    let mut handles = vec![];
    for _ in 0..MAX_THREADS {
        let world = world.clone();
        let camera = camera.clone();
        let r = r.clone();
        let canvas = canvas.clone();
        let finished = finished.clone();
        let h = std::thread::spawn(move || {
            let mut rng = rand::thread_rng();
            while let Ok((i, j)) = r.recv() {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                    let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    let sample_color = ray_color(ray, world.clone(), MAX_DEPTH);
                    color += sample_color;
                }
                canvas.lock().unwrap()[j][i] = color / SAMPLES_PER_PIXEL as f64;

                finished.lock().unwrap()[j] += 1;
                if finished.lock().unwrap()[j] == image_width {
                    eprintln!("\r Scanline remaining: {}", j);
                }
            }
        });
        handles.push(h);
    }

    drop(s);
    for h in handles {
        h.join().unwrap();
    }

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            println!("{}", canvas.lock().unwrap()[j][i]);
        }
    }
}

fn ray_color(ray: Ray, world: Arc<dyn Hittable>, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(&ray, &RangeInclusive::new(0.001, std::f64::INFINITY)) {
        if let Some((attenuation, scattered_ray)) = hit.material.scatter(&ray, &hit) {
            return attenuation * ray_color(scattered_ray, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let t = (ray.direction().normalize().y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
