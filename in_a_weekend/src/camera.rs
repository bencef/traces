use crate::{p3::Point3, v3::Vec3};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ASPECT_RATIO;
        let focal_length = 1.0;

        let origin = Point3::zero();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin
            - horizontal.scale(0.5)
            - vertical.scale(0.5)
            - Vec3::new(0.0, 0.0, focal_length);

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
