use crate::{
    materials::Material,
    vec3::{Point3D, Vec3},
};

#[derive(Clone, Copy)]
/// Returns a ray represented by an origin in space and a direction.
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn point_ray(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub struct HitRecord<'material> {
    pub point: Point3D,
    pub normal: Vec3,
    pub outward_facing: bool,
    pub t: f32,
    pub mtrl: &'material Material,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
