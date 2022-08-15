use super::{Material, Scatter};
use crate::{color::Color, hittable::HitRecord, ray::Ray};
use std::rc::Rc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Rc<Self> {
        Rc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, rec: &HitRecord) -> Option<Scatter> {
        todo!()
    }
}
