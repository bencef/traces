use super::HitRecord;
use crate::{color::Color, ray::Ray, v3::Vec3};

pub mod lambertian;
pub mod metal;
pub mod dielectric;

#[derive(Debug)]
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

fn reflect(incoming: Vec3, normal: Vec3) -> Vec3 {
    incoming - normal.scale(2.0 * Vec3::dot(incoming, normal))
}
