use crate::v3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Color(Vec3);

const SCALE_TO_256: f64 = 255.999f64;

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(r, g, b))
    }

    // TODO: this should be in Ppm
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
