use crate::vec3::{Vec3, Point3};

#[derive(Clone, Copy)]
/// Returns a ray represented by an origin in space and a direction.
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    
    //pub fn point_ray(self, t: f32) -> Vec3 {
    //    self.origin + t * self.direction
    //
}
