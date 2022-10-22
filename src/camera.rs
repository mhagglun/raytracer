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
    pub fn new(
        viewfrom: Point3D,
        viewat: Point3D,
        vup: Vec3,
        fov: f32,
        aspect_ratio: f32,
    ) -> Camera {
        // Vertical field-of-view in degrees
        let theta = std::f32::consts::PI / 180.0 * fov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (viewfrom - viewat).unit_vector();
        let cu = vup.cross(cw).unit_vector();
        let cv = cw.cross(cu);

        let h = viewport_width * cu;
        let v = viewport_height * cv;

        let llc = viewfrom - h / 2.0 - v / 2.0 - cw;

        Camera {
            origin: viewfrom,
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
