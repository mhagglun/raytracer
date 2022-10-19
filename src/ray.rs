use crate::{
    sphere::{hit_sphere, Sphere},
    vec3::{Color, Point3D, Vec3},
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

/// Linearly blends the color depending on the height of the y-coordinate after scaling the ray
/// direction to unit length
pub fn ray_color(sphere: Sphere, ray: Ray) -> Color {
    let t = hit_sphere(sphere, ray);
    if t > 0.0 {
        let n = ray.point_ray(t);
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
