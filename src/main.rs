mod color;
mod p3;
mod ppm;
mod ray;
mod v3;

use color::Color;
use p3::Point3;
use ppm::Ppm;
use ray::Ray;
use v3::Vec3;

pub struct Rect {
    width: usize,
    height: usize,
}

fn ray_color(r: Ray) -> Color {
    let dir = r.dir().normalized();
    let t = 0.5 * (dir.y() + 1.0);
    let mix_factor_white = 1.0 - t;
    let mix_factor_sky = t;
    mix_factor_white * Color::rgb(1.0, 1.0, 1.0)
        + mix_factor_sky * Color::rgb(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - horizontal.scale(0.5)
        - vertical.scale(0.5)
        - Vec3::new(0.0, 0.0, focal_length);
    let ppm = Ppm::new(Rect {
        height: IMAGE_HEIGHT,
        width: IMAGE_WIDTH,
    });
    let color_for_position = move |Rect { width, height }| {
        let u = width as f64 / (IMAGE_WIDTH - 1) as f64;
        let v = height as f64 / (IMAGE_HEIGHT - 1) as f64;
        let dir = (lower_left_corner - origin) + horizontal.scale(u) + vertical.scale(v);
        let r = Ray::new(origin, dir);
        ray_color(r)
    };
    ppm.write(&mut std::io::stdout(), color_for_position)
}
