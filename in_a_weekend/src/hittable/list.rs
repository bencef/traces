use super::Hittable;
use std::{sync::Arc, vec::Vec};

pub struct HittableList {
    hittables: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittables: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.hittables.clear()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.hittables.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        let mut closest = t_max;
        let mut closest_hit = None;

        for object in self.hittables.iter() {
            // closest is passed as `t_max'
            if let Some(rec) = object.as_ref().hit(ray, t_min, closest) {
                closest = rec.scale;
                closest_hit = Some(rec);
            }
        }

        closest_hit
    }
}
