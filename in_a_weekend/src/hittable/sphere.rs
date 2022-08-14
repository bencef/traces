use super::{HitRecord, Hittable};
use crate::{p3::Point3, ray::Ray, v3::Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        // variables for quadratic equation
        let a = Vec3::dot(r.dir(), r.dir());
        let half_b = Vec3::dot(oc, r.dir());
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

        let point = r.at(root);
        let scale = root;
        let normal = (point - self.center).scale(1.0 / self.radius);

        let rec = HitRecord {
            point,
            normal,
            scale,
        };
        Some(rec)
    }
}
