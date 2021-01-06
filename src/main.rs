mod p3;
mod ray;
mod v3;

use p3::Point3;
use ray::Ray;
use v3::Vec3;

#[derive(Clone, Copy, Debug)]
struct Color(Vec3);

const SCALE_TO_256: f64 = 255.999f64;

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(r, g, b))
    }

    pub fn write<W>(self: &Self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let ir = (self.0.x() * SCALE_TO_256) as u8;
        let ig = (self.0.y() * SCALE_TO_256) as u8;
        let ib = (self.0.z() * SCALE_TO_256) as u8;
        writeln!(writer, "{} {} {}", ir, ig, ib)
    }

    pub fn scale(self: Self, amount: f64) -> Self {
        let v = self.0.scale(amount);
        Color(v)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self: Self, rhs: Color) -> Self::Output {
        rhs.scale(self)
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self: Self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

struct Rect {
    width: usize,
    height: usize,
}

struct Ppm {
    size: Rect,
}

impl Ppm {
    pub fn new(size: Rect) -> Self {
        Ppm { size }
    }

    pub fn write<W, F>(self: &Self, writer: &mut W, get_color: F) -> std::io::Result<()>
    where
        W: std::io::Write,
        F: Fn(Rect) -> Color,
    {
        writer.write_all(b"P3\n")?;
        writeln!(writer, "{} {}", self.size.width, self.size.height)?;
        writer.write_all(b"255\n")?;
        for height in (0..self.size.width).rev() {
            for width in 0..self.size.width {
                let color = get_color(Rect { width, height });
                color.write(writer)?;
            }
        }
        writer.flush()
    }
}

fn ray_color(r: Ray) -> Color {
    let dir = r.dir().normalized();
    let t = 0.5f64 * (dir.y() + 1f64);
    let mix_factor_white = 1f64 - t;
    let mix_factor_sky = t;
    mix_factor_white * Color::rgb(1f64, 1f64, 1f64)
        + mix_factor_sky * Color::rgb(0.5f64, 0.7f64, 1f64)
}

fn main() -> std::io::Result<()> {
    const ASPECT_RATIO: f64 = 16f64 / 9f64;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let viewport_height = 2f64;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1f64;

    let origin = Point3::new(0f64, 0f64, 0f64);
    let horizontal = Vec3::new(viewport_width, 0f64, 0f64);
    let vertical = Vec3::new(0f64, viewport_height, 0f64);
    let lower_left_corner = origin
        - horizontal.scale(0.5f64)
        - vertical.scale(0.5f64)
        - Vec3::new(0f64, 0f64, focal_length);
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
