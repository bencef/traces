use std::rc::Rc;

use super::{material::Material, HitRecord, Hittable};
use crate::{p3::Point3, ray::Ray, v3::Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn new_rc(center: Point3, radius: f64, material: Rc<dyn Material>) -> Rc<Self> {
        Rc::new(Self::new(center, radius, material))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        // variables for quadratic equation
        let a = Vec3::dot(ray.dir(), ray.dir());
        let half_b = Vec3::dot(oc, ray.dir());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // ray intersects sphere
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // find nearest root in acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            // check other root
            root = (-half_b + sqrt_d) / a;
        }
        if root < t_min || root > t_max {
            // none of the roots within range
            return None;
        }

        let normal = (ray.at(root) - self.center).scale(1.0 / self.radius);

        let rec = HitRecord::new(ray, normal, self.material.clone(), root);

        Some(rec)
    }
}
