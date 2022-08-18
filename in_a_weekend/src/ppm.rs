use crate::color::Color;
use crate::Rect;

#[derive(Clone)]
pub struct Ppm {
    size: Rect,
}

impl Ppm {
    pub fn new(size: Rect) -> Self {
        Ppm { size }
    }

    pub fn write<W, F>(&self, writer: &mut W, get_color: F) -> std::io::Result<()>
    where
        W: std::io::Write,
        F: Fn(Rect) -> Color,
    {
        writer.write_all(b"P3\n")?;
        writeln!(writer, "{} {}", self.size.width, self.size.height)?;
        writer.write_all(b"255\n")?;
        for height in (0..self.size.height).rev() {
            for width in 0..self.size.width {
                let color = get_color(Rect { width, height });
                write_color(&color, writer)?;
            }
        }
        writer.flush()
    }
}

fn write_color<W>(color: &Color, writer: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    const SCALE_TO_256: f64 = 255.999f64;
    let ir = (color.r() * SCALE_TO_256) as u8;
    let ig = (color.g() * SCALE_TO_256) as u8;
    let ib = (color.b() * SCALE_TO_256) as u8;
    writeln!(writer, "{} {} {}", ir, ig, ib)
}
