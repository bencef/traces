mod camera;
mod color;
mod hittable;
mod p3;
mod ppm;
mod ray;
mod v3;

use color::Color;
use hittable::Hittable;
use p3::Point3;
use ppm::Ppm;
use rand::Rng;
use ray::Ray;
use std::{f64::{consts::PI, INFINITY}, sync::{Arc, RwLock}};
use v3::Vec3;

use crate::{
    camera::Camera,
    hittable::{
        list::HittableList,
        material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
        sphere::Sphere,
    },
};

#[derive(Clone)]
pub struct Rect {
    width: usize,
    height: usize,
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::rgb(0.0, 0.0, 0.0);
    }

    // eprintln!("Ray {:?} at depth: {}", *ray, depth);
    // avoid self bounces
    let min = 0.001;
    if let Some(rec) = world.hit(ray, min, INFINITY) {
        // eprintln!("There was a hit for {:?}", rec);
        if let Some(scatter) = rec.material().as_ref().scatter(ray, &rec) {
            // eprintln!("Scattered ray is: {:?}", scatter);
            return scatter.attenuation() * ray_color(scatter.ray(), world, depth - 1);
        }
        // There was a hit, but it is absorbed
        // eprintln!("Ray was absorbed");
        return Color::rgb(0.0, 0.0, 0.0);
    }
    // eprint!("No hit for ray");
    let dir = ray.dir().normalized();
    let t = 0.5 * (dir.y() + 1.0);
    let mix_factor_sky_bottom = 1.0 - t;
    let mix_factor_sky_top = t;
    let sky_bottom_color = Color::rgb(1.0, 1.0, 1.0).into_gamma();
    let sky_top_color = Color::rgb(0.5, 0.7, 1.0).into_gamma();
    mix_factor_sky_bottom * sky_bottom_color + mix_factor_sky_top * sky_top_color
}

#[cfg(debug_assertions)]
const SAMPLE_PER_PIXEL: usize = 10;
#[cfg(not(debug_assertions))]
const SAMPLE_PER_PIXEL: usize = 100;

/// Number of ray bounces to calculate
const MAX_DEPTH: usize = 50;

#[cfg(debug_assertions)]
const IMAGE_WIDTH: usize = 400;
#[cfg(not(debug_assertions))]
const IMAGE_WIDTH: usize = 1080;

const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / camera::ASPECT_RATIO) as usize;

const WORKER_THREADS: usize = 8;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    let matte_ground = Lambertian::new_arc(Color::rgb(0.8, 0.8, 0.0));
    let glass_center = Dielectric::new_arc(1.5);
    let metal_left = Metal::new_arc(Color::rgb(0.8, 0.8, 0.8), 0.05);
    let metal_right = Metal::new_arc(Color::rgb(0.8, 0.6, 0.2), 0.7);

    world.add(Sphere::new_arc(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        matte_ground,
    ));
    world.add(Sphere::new_arc(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        glass_center,
    ));
    world.add(Sphere::new_arc(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        metal_left,
    ));
    world.add(Sphere::new_arc(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        metal_right,
    ));

    let ppm = Ppm::new(Rect {
        height: IMAGE_HEIGHT,
        width: IMAGE_WIDTH,
    });

    eprintln!("Using samples per pixel: {}", SAMPLE_PER_PIXEL);

    let camera_focus = Point3::new(0.0, 0.0, -1.0);
    let cam_radius = 2.0;

    const FPS: usize = 60;
    const SCENE_LEN_SEC: usize = 3;
    const FRAMES: usize = FPS * SCENE_LEN_SEC;

    let pool = threadpool::ThreadPool::new(WORKER_THREADS);
    let world = Arc::new(RwLock::new(world));

    for frame in 0..FRAMES {
        let tau = frame as f64 * 2.0 * PI / FRAMES as f64;
        let origin = Point3::new(f64::sin(tau) * cam_radius, 0.0, f64::cos(tau) * cam_radius)
            + camera_focus.into()
            + Vec3::new(0.0, 0.5, 0.0);
        let camera = Camera::new(origin, camera_focus, Vec3::new(0.0, 1.0, 0.0));

        let ppm = ppm.clone();
        let world = world.clone();

        pool.execute(move || {
            let color_for_position = move |Rect { width, height }| {
                let world = world.as_ref().read().expect("Couldn't read world");
                let mut rng = rand::thread_rng();
                let mut color = Color::rgb(0.0, 0.0, 0.0);
                for _sample_number in 1..SAMPLE_PER_PIXEL {
                    let u = (width as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (height as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let dir = camera.dir(u, v);
                    let r = Ray::new(camera.origin(), dir);
                    color += ray_color(&r, &*world, MAX_DEPTH);
                }
                color.sampled(SAMPLE_PER_PIXEL).gamma_corrected()
            };

            let out_file_name = format!("out_{:05}.ppm", frame);
            let mut out_file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(out_file_name.clone())
                .expect(&format!("Can't open file for writing: {}", out_file_name));
            ppm.write(&mut out_file, color_for_position)
                .expect(&format!("Couldn't write PPM for: {}", out_file_name))
            // eprintln!("{:?}", ray_color(&Ray::new(camera.origin(), camera.dir(0.5, 0.5)), &world, MAX_DEPTH));
        });
    }
    pool.join();
    Ok(())
}
