use crate::{ray::Ray, vec3::*};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizonal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin_t = Point3::from(0f64, 0f64, 0f64);

        let horizonal_t = Vec3::from(viewport_width, 0.0, 0.0);
        let vertical_t = Vec3::from(0.0, viewport_height, 0.0);

        Camera {
            origin: origin_t,
            lower_left_corner: origin_t
                - horizonal_t / 2.0
                - vertical_t / 2.0
                - Vec3::from(0.0, 0.0, focal_length),
            vertical: vertical_t,
            horizonal: horizonal_t,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::from(
            self.origin,
            self.lower_left_corner + u * self.horizonal + v * self.vertical - self.origin,
        )
    }
}
