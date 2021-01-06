mod v3;

#[derive(Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn write<W>(self: Self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let (ir, ig, ib) = match self {
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 0, 255),
        };
        writeln!(writer, "{} {} {}", ir, ig, ib)
    }
}

struct Ppm {
    // TODO: dynamic size
    pixels: [[Color; 256]; 256],
}

impl Ppm {
    pub fn new() -> Self {
        let line = [Color::Green; 256];
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
