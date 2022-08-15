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
use std::{f64::INFINITY, rc::Rc};
use v3::Vec3;

use crate::{
    camera::Camera,
    hittable::{list::HittableList, sphere::Sphere},
};

pub struct Rect {
    width: usize,
    height: usize,
}

fn ray_color(ray: Ray, world: &dyn Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::rgb(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(&ray, 0.0, INFINITY) {
        let target = rec.point() + rec.normal() + random_in_unit_sphere();
        let ray = Ray::new(rec.point(), target - rec.point());
        return 0.5 * ray_color(ray, world, depth - 1);
    }
    let dir = ray.dir().normalized();
    let t = 0.5 * (dir.y() + 1.0);
    let mix_factor_white = 1.0 - t;
    let mix_factor_sky = t;
    mix_factor_white * Color::rgb(1.0, 1.0, 1.0) + mix_factor_sky * Color::rgb(0.5, 0.7, 1.0)
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let vec = Vec3::random(-1.0, 1.0);
        // FIXME: less is used in the book.  Isn't equal OK too?
        if vec.length_squared() < 1.0 {
            return vec;
        }
    }
}

#[cfg(debug_assertions)]
const SAMPLE_PER_PIXEL: usize = 10;

#[cfg(not(debug_assertions))]
const SAMPLE_PER_PIXEL: usize = 100;

const MAX_DEPTH: usize = 50;

fn main() -> std::io::Result<()> {
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / camera::ASPECT_RATIO) as usize;

    let camera = Camera::new();

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
            color += ray_color(r, &world, MAX_DEPTH);
        }
        color.sampled(SAMPLE_PER_PIXEL)
    };
    ppm.write(&mut std::io::stdout(), color_for_position)
}
