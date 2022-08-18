use self::material::Material;
use crate::{Point3, Ray, Vec3};

use std::rc::Rc;

pub mod list;
pub mod material;
pub mod sphere;

pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    material: Rc<dyn Material>,
    scale: f64,
    front_face: bool,
}

impl std::fmt::Debug for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HitRecord")
            .field("point", &self.point)
            .field("normal", &self.normal)
            // .field("material", &self.material)
            .field("scale", &self.scale)
            .field("front_face", &self.front_face)
            .finish()
    }
}

impl HitRecord {
    pub fn new(ray: &Ray, normal: Vec3, material: Rc<dyn Material>, scale: f64) -> Self {
        let point = ray.at(scale);
        let front_face = Vec3::dot(ray.dir(), normal) < 0.0;
        let normal = if front_face {
            normal
        } else {
            normal.scale(-1.0)
        };
        Self {
            point,
            normal,
            material,
            scale,
            front_face,
        }
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }

    fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
