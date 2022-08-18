use crate::{p3::Point3, v3::Vec3};

pub const ASPECT_RATIO: f64 = 1.0;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, view_up: Vec3) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ASPECT_RATIO;

        let w = (look_from - look_at).normalized();
        let u = Vec3::cross(view_up, w).normalized();
        let v = Vec3::cross(w, u);

        let origin = look_from;
        let horizontal = u.scale(viewport_width);
        let vertical = v.scale(viewport_height);
        let lower_left_corner = origin - horizontal.scale(0.5) - vertical.scale(0.5) - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn dir(&self, u: f64, v: f64) -> Vec3 {
        (self.lower_left_corner - self.origin) + self.horizontal.scale(u) + self.vertical.scale(v)
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }
}
