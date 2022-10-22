use crate::{
    ray::Ray,
    vec3::{Point3D, Vec3},
};

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vec3,
    vertial: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f32 = 2.0;
        const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.0;

        let orig = Point3D::new(0.0, 0.0, 0.0);
        let h = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let v = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let llc = orig - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera {
            origin: orig,
            horizontal: h,
            vertial: v,
            lower_left_corner: llc,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertial - self.origin,
        )
    }
}
