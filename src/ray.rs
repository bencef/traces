use crate::p3::Point3;
use crate::v3::Vec3;

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn at(self: &Self, t: f64) -> Point3 {
        self.orig + self.dir.scale(t)
    }

    pub fn dir(self: &Self) -> Vec3 {
        self.dir
    }
}
