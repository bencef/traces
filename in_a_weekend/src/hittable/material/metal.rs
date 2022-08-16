use std::rc::Rc;

use super::{Material, Scatter};
use crate::{color::Color, hittable::HitRecord, ray::Ray, v3::Vec3};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new_rc(albedo: Color) -> Rc<Self> {
        Rc::new(Self { albedo })
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(ray.dir().normalized(), rec.normal());
        let scattered_ray = Ray::new(rec.point(), reflected);
        let attenuation = self.albedo;
        if Vec3::dot(scattered_ray.dir(), rec.normal()) > 0.0 {
            Some(Scatter {
                scattered_ray,
                attenuation,
            })
        } else {
            None
        }
    }
}

fn reflect(incoming: Vec3, normal: Vec3) -> Vec3 {
    incoming - normal.scale(2.0 * Vec3::dot(incoming, normal))
}
