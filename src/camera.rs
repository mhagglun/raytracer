use crate::{
    ray::Ray,
    vec3::{Point3D, Vec3},
};

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        viewfrom: Point3D,
        viewat: Point3D,
        vup: Vec3,
        fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        // Vertical field-of-view in degrees
        let theta = std::f32::consts::PI / 180.0 * fov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (viewfrom - viewat).unit_vector();
        let cu = vup.cross(cw).unit_vector();
        let cv = cw.cross(cu);

        let h = focus_dist * viewport_width * cu;
        let v = focus_dist * viewport_height * cv;

        let llc = viewfrom - h / 2.0 - v / 2.0 - focus_dist * cw;

        Camera {
            origin: viewfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
            cu,
            cv,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_sphere();
        let offset = self.cu * rd + self.cv * rd;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
