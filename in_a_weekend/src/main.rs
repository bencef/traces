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
use std::f64::INFINITY;
use v3::Vec3;

use crate::{
    camera::Camera,
    hittable::{
        list::HittableList,
        material::{lambertian::Lambertian, metal::Metal},
        sphere::Sphere,
    },
};

pub struct Rect {
    width: usize,
    height: usize,
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::rgb(0.0, 0.0, 0.0);
    }

    // avoid self bounces
    let min = 0.001;
    if let Some(rec) = world.hit(ray, min, INFINITY) {
        if let Some(scatter) = rec.material().as_ref().scatter(ray, &rec) {
            return scatter.attenuation() * ray_color(scatter.ray(), world, depth - 1);
        }
        // There was a hit, but it is absorbed
        return Color::rgb(0.0, 0.0, 0.0);
    }
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
const IMAGE_WIDTH: usize = 1000;

const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / camera::ASPECT_RATIO) as usize;

fn main() -> std::io::Result<()> {
    let camera = Camera::new();

    let mut world = HittableList::new();
    let matte_ground = Lambertian::new_rc(Color::rgb(0.8, 0.8, 0.0));
    let matte_center = Lambertian::new_rc(Color::rgb(0.7, 0.3, 0.3));
    let metal_left = Metal::new_rc(Color::rgb(0.8, 0.8, 0.8), 0.05);
    let metal_right = Metal::new_rc(Color::rgb(0.8, 0.6, 0.2), 0.7);

    world.add(Sphere::new_rc(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        matte_ground,
    ));
    world.add(Sphere::new_rc(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        matte_center,
    ));
    world.add(Sphere::new_rc(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        metal_left,
    ));
    world.add(Sphere::new_rc(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        metal_right,
    ));

    let ppm = Ppm::new(Rect {
        height: IMAGE_HEIGHT,
        width: IMAGE_WIDTH,
    });

    eprintln!("Using samples per pixel: {}", SAMPLE_PER_PIXEL);

    let color_for_position = move |Rect { width, height }| {
        let mut rng = rand::thread_rng();
        let mut color = Color::rgb(0.0, 0.0, 0.0);
        for _sample_number in 1..SAMPLE_PER_PIXEL {
            let u = (width as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
            let v = (height as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
            let dir = camera.dir(u, v);
            let r = Ray::new(camera.origin(), dir);
            color += ray_color(&r, &world, MAX_DEPTH);
        }
        color.sampled(SAMPLE_PER_PIXEL).gamma_corrected()
    };
    ppm.write(&mut std::io::stdout(), color_for_position)
}
