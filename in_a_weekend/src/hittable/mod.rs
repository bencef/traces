use crate::{Point3, Ray, Vec3};

pub mod sphere;
pub mod list;

pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    scale: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, normal: Vec3, scale: f64) -> Self {
        let point = ray.at(scale);
        // BUG: Book says this should be less than 0
        let front_face = Vec3::dot(ray.dir(), ray.dir()) > 0.0;
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

    pub(crate) fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
