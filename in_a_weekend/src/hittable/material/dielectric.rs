use std::rc::Rc;

use crate::{color::Color, hittable::HitRecord, ray::Ray, v3::Vec3};

use super::{Material, Scatter};

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
        if let Some(refracted_ray) = refract(
            unit_ray_direction,
            rec.normal().normalized(),
            refraction_ratio,
        ) {
            let scattered_ray = Ray::new(rec.point(), refracted_ray);

            Some(Scatter {
                scattered_ray,
                attenuation,
            })
        } else {
            // miss reflection
            None
        }
    }
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
