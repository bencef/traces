use crate::v3::Vec3;

struct Point3(Vec3);

struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn at(self: &Self, t: f64) -> Vec3 {
        self.orig.0 + self.dir.scale(t)
    }
}
