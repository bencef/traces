use super::{Material, Scatter};
use crate::{color::Color, hittable::HitRecord, ray::Ray, v3::Vec3};
use std::rc::Rc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new_rc(albedo: Color) -> Rc<Self> {
        Rc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        // scattering is random so the incoming ray is not used
        let mut scatter_direction = rec.normal() + random_unit_vector();

        // Catch degenerate scatter directions
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        let scattered_ray = Ray::new(rec.point(), scatter_direction);
        let attenuation = self.albedo;
        Some(Scatter {
            scattered_ray,
            attenuation,
        })
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalized()
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let vec = Vec3::random(-1.0, 1.0);
        // FIXME: less is used in the book.  Isn't equal OK too?
        if vec.length_squared() < 1.0 {
            return vec;
        }
    }
}
