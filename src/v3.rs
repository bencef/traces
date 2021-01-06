use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    e1: f64,
    e2: f64,
    e3: f64,
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Vec3 { e1, e2, e3 }
    }

    pub fn zero() -> Self {
        Vec3 {
            e1: 0f64,
            e2: 0f64,
            e3: 0f64,
        }
    }

    pub fn scale(self: Self, factor: f64) -> Self {
        let e1 = self.e1 * factor;
        let e2 = self.e2 * factor;
        let e3 = self.e3 * factor;
        Vec3 { e1, e2, e3 }
    }

    pub fn normalized(self: Self) -> Self {
        let one_over_size = 1f64 / self.size();
        self.scale(one_over_size)
    }

    pub fn size(self: &Self) -> f64 {
        let squared = self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3;
        squared.sqrt()
    }

    // x, y, z
    pub fn x(self: &Self) -> f64 {
        self.e1
    }

    pub fn y(self: &Self) -> f64 {
        self.e2
    }

    pub fn z(self: &Self) -> f64 {
        self.e3
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self: Self) -> Self::Output {
        let e1 = -self.e1;
        let e2 = -self.e2;
        let e3 = -self.e3;
        Vec3 { e1, e2, e3 }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self: Self, other: Self) -> Self::Output {
        let e1 = self.e1 + other.e1;
        let e2 = self.e2 + other.e2;
        let e3 = self.e3 + other.e3;
        Vec3 { e1, e2, e3 }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self: Self, rhs: Self) -> Self::Output {
        let e1 = self.e1 - rhs.e1;
        let e2 = self.e2 - rhs.e2;
        let e3 = self.e3 - rhs.e3;

        Vec3 { e1, e2, e3 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn negates_add_up_to_zero() {
        let v = Vec3::new(1f64, 0f64, -1f64);
        let v_neg = -v.clone();
        assert_eq!(v + v_neg, Vec3::zero());
    }
}
