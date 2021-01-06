mod v3;

#[derive(Clone, Copy)]
struct Color(v3::Vec3);

const SCALE_TO_256: f64 = 255.999f64;

impl Color {
    pub fn write<W>(self: &Self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let ir = (self.0.x() * SCALE_TO_256) as u8;
        let ig = (self.0.y() * SCALE_TO_256) as u8;
        let ib = (self.0.z() * SCALE_TO_256) as u8;
        writeln!(writer, "{} {} {}", ir, ig, ib)
    }

    pub fn green() -> Self {
        Color(v3::Vec3::new(0f64, 1f64, 0f64))
    }
}

struct Ppm {
    // TODO: dynamic size
    pixels: [[Color; 256]; 256],
}

impl Ppm {
    pub fn new() -> Self {
        let line = [Color::green(); 256];
        let pixels = [line; 256];
        Ppm { pixels }
    }

    pub fn write<W>(self: &Self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(b"P3\n")?;
        writer.write_all(b"256 256\n")?;
        writer.write_all(b"255\n")?;
        for &line in self.pixels.iter() {
            for &pixel in line.iter() {
                pixel.write(writer)?;
            }
        }
        writer.flush()
    }
}

fn main() -> std::io::Result<()> {
    let ppm = Ppm::new();
    ppm.write(&mut std::io::stdout())
}
