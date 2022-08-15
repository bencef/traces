use super::HitRecord;
use crate::{color::Color, ray::Ray};

pub mod lambertian;

pub struct Scatter {
    scattered_ray: Ray,
    attenuation: Color,
}

impl Scatter {
    pub fn attenuation(&self) -> Color {
        self.attenuation
    }

    pub fn ray(&self) -> &Ray {
        &self.scattered_ray
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter>;
}
