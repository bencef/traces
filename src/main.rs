#[derive(Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
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

    pub fn print(self: &Self) {
        println!("P3");
        println!("256 256");
        println!("255");
        for &line in self.pixels.iter() {
            for &pixel in line.iter() {
                let (ir, ig, ib) = match pixel {
                    Color::Red => (255, 0, 0),
                    Color::Green => (0, 255, 0),
                    Color::Blue => (0, 0, 255),
                };
                println!("{} {} {}", ir, ig, ib);
            }
        }
    }
}

fn main() {
    let ppm = Ppm::new();
    ppm.print();
}
