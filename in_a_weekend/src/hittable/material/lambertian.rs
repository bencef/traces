use super::{random_unit_vector, Material, Scatter};
use crate::{color::Color, hittable::HitRecord, ray::Ray};
use std::sync::Arc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new_arc(albedo: Color) -> Arc<Self> {
        Arc::new(Self { albedo })
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
