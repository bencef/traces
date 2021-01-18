use crate::Rect;
use crate::color::Color;

pub struct Ppm {
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
        for height in (0..self.size.height).rev() {
            for width in 0..self.size.width {
                let color = get_color(Rect { width, height });
                color.write(writer)?;
            }
        }
        writer.flush()
    }
}
