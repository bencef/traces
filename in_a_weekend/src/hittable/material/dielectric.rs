use std::rc::Rc;

use rand::Rng;

use crate::{color::Color, hittable::HitRecord, ray::Ray, v3::Vec3};

use super::{reflect, Material, Scatter};

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new_rc(refraction_index: f64) -> Rc<Self> {
        Rc::new(Self { refraction_index })
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = Color::rgb(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_ray_direction = ray.dir().normalized();
        let ray_direction = match refract(unit_ray_direction, rec.normal(), refraction_ratio)
            .filter(|_ray| !should_reflect(unit_ray_direction, rec.normal(), refraction_ratio))
        {
            Some(refracted_ray) => refracted_ray,
            None => reflect(unit_ray_direction, rec.normal()),
        };

        let scattered_ray = Ray::new(rec.point(), ray_direction);
        Some(Scatter {
            scattered_ray,
            attenuation,
        })
    }
}

fn should_reflect(ray_dir: Vec3, normal: Vec3, refraction_ratio: f64) -> bool {
    let cos_theta = Vec3::dot(ray_dir, normal.scale(-1.0)).min(1.0);
    reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen()
}

fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
    // Use Schlick's approximation
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

fn refract(unit_ray_direction: Vec3, normal: Vec3, refraction_ratio: f64) -> Option<Vec3> {
    let cos_theta_one = Vec3::dot(unit_ray_direction, normal.scale(-1.0));
    if cos_theta_one > 1.0 {
        return None;
    }
    let sin_theta_two = refraction_ratio * (1.0 - cos_theta_one.powi(2)).sqrt();
    let cos_theta_two = (1.0 - sin_theta_two.powi(2)).sqrt();
    Some(
        unit_ray_direction.scale(refraction_ratio)
            + normal.scale(refraction_ratio * cos_theta_one - cos_theta_two),
    )
}
