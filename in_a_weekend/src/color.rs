use crate::v3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Color(Vec3);

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(r, g, b))
    }

    pub fn scale(self, amount: f64) -> Self {
        let v = self.0.scale(amount);
        Color(v)
    }

    pub fn from_normal(n: Vec3) -> Self {
        let moved = n + Vec3::new(1.0, 1.0, 1.0);
        let scaled = moved.scale(0.5);
        Self(scaled)
    }

    pub fn r(&self) -> f64 {
        self.0.x()
    }

    pub fn g(&self) -> f64 {
        self.0.y()
    }

    pub fn b(&self) -> f64 {
        self.0.z()
    }

    pub fn sampled(self, sample_per_pixel: usize) -> Self {
        let (e1, e2, e3) = self.0.xyz();
        let e1 = f64::clamp(e1 / sample_per_pixel as f64, 0.0, 1.0);
        let e2 = f64::clamp(e2 / sample_per_pixel as f64, 0.0, 1.0);
        let e3 = f64::clamp(e3 / sample_per_pixel as f64, 0.0, 1.0);
        let v = Vec3::new(e1, e2, e3);
        Self(v)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs.scale(self)
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}
