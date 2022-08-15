use rand::Rng;
use std::ops::*;

use crate::p3::Point3;

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

    pub fn dot(a: Self, b: Self) -> f64 {
        a.e1 * b.e1 + a.e2 * b.e2 + a.e3 * b.e3
    }

    pub fn scale(self, factor: f64) -> Self {
        let e1 = self.e1 * factor;
        let e2 = self.e2 * factor;
        let e3 = self.e3 * factor;
        Vec3 { e1, e2, e3 }
    }

    pub fn normalized(self) -> Self {
        let one_over_size = 1f64 / self.size();
        self.scale(one_over_size)
    }

    pub fn size(self) -> f64 {
        let squared = self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3;
        squared.sqrt()
    }

    pub fn x(&self) -> f64 {
        self.e1
    }

    pub fn y(&self) -> f64 {
        self.e2
    }

    pub fn z(&self) -> f64 {
        self.e3
    }

    pub fn xyz(&self) -> (f64, f64, f64) {
        (self.x(), self.y(), self.z())
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let e1 = lerp(min, max, rng.gen::<f64>());
        let e2 = lerp(min, max, rng.gen::<f64>());
        let e3 = lerp(min, max, rng.gen::<f64>());
        Self::new(e1, e2, e3)
    }
}

// FIXME: rust's f64::lerp is unstable at the time of writing this
fn lerp(min: f64, max: f64, t: f64) -> f64 {
    min + t * (max - min)
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let e1 = -self.e1;
        let e2 = -self.e2;
        let e3 = -self.e3;
        Vec3 { e1, e2, e3 }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let e1 = self.e1 + other.e1;
        let e2 = self.e2 + other.e2;
        let e3 = self.e3 + other.e3;
        Vec3 { e1, e2, e3 }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let e1 = self.e1 - rhs.e1;
        let e2 = self.e2 - rhs.e2;
        let e3 = self.e3 - rhs.e3;

        Vec3 { e1, e2, e3 }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e1 += fix_nan(rhs.e1);
        self.e2 += fix_nan(rhs.e2);
        self.e3 += fix_nan(rhs.e3);
    }
}

fn fix_nan(v: f64) -> f64 {
    if v.is_nan() {
        0.0
    } else {
        v
    }
}

impl From<Point3> for Vec3 {
    fn from(p: Point3) -> Self {
        let (x, y, z) = p.xyz();
        Self {
            e1: x,
            e2: y,
            e3: z,
        }
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

    #[test]
    pub fn accummulating_nan_doesnt_kill() {
        let mut v = Vec3::zero();
        let with_nan = Vec3::new(f64::NAN, 0.0, 0.0);
        v += with_nan;
        assert_eq!(v.x(), 0.0);
    }

    #[test]
    pub fn lerp_does_what_i_think_it_does() {
        let lerp_min = lerp(-2.0, 2.0, 0.0);
        let lerp_max = lerp(-2.0, 2.0, 1.0);
        fn close_enough(a: f64, b: f64) -> bool {
            (a - b).abs() < 1e-10
        }
        assert!(close_enough(lerp_min, -2.0));
        assert!(close_enough(lerp_max, 2.0));
    }
}
