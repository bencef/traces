use super::HitRecord;
use crate::{color::Color, ray::Ray};

pub mod lambertian;

pub struct Scatter {
    scattered_ray: Ray,
    attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: Ray, rec: &HitRecord) -> Option<Scatter>;
}
