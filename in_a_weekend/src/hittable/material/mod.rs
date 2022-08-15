use crate::{color::Color, ray::Ray};

use super::HitRecord;

pub trait Material {
    fn scatter(&self, ray: Ray, rec: &HitRecord, attenuation: &Color) -> Option<Ray>;
}
