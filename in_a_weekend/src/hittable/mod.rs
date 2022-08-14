use crate::{Point3, Ray, Vec3};

mod sphere;

pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    scale: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
