use crate::{Point3, Ray, Vec3};

mod sphere;

pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    scale: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, normal: Vec3, scale: f64) -> Self {
        let point = ray.at(scale);
        let front_face = Vec3::dot(ray.dir(), ray.dir()) < 0.0;
        let normal = if front_face {
            normal
        } else {
            normal.scale(-1.0)
        };
        Self {
            point,
            normal,
            scale,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
