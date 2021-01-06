use crate::v3::Vec3;

use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Point3(Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3(Vec3::new(x, y, z))
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self: Self, rhs: Vec3) -> Self::Output {
        Point3(self.0 - rhs)
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self: Self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self: Self, rhs: Vec3) -> Self::Output {
        Point3(self.0 + rhs)
    }
}
