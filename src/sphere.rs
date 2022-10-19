use crate::{ray::Ray, vec3::Point3D};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

pub fn hit_sphere(sphere: Sphere, ray: Ray) -> f32 {
    let oc = ray.origin - sphere.center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - sphere.radius.powf(2.0);
    let discriminant = half_b.powf(2.0) - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

#[test]
fn test_sphere_hit() {
    let center = Point3D::new(0.0, 0.0, 0.0);
    let sphere = Sphere::new(center, 1.0);
    let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(0.0, 0.0, 1.0));
    let hit = hit_sphere(sphere, ray);
    assert_eq!(hit, -1.0);
}
