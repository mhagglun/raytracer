use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3D;

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

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant >= 0.0 {
            // Find the nearest root that lies in the acceptable range
            let sqrtd = discriminant.sqrt();
            let root_a = (-half_b - sqrtd) / a;
            let root_b = (-half_b + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < t_max && *root > t_min {
                    let p = ray.point_ray(*root);
                    let normal = (1.0 / self.radius) * (p - self.center);
                    let front_face = ray.direction.dot(normal) < 0.0;

                    return Some(HitRecord {
                        t: *root,
                        point: p,
                        normal: if front_face { normal } else { -normal },
                    });
                }
            }
        }
        None
    }
}

#[test]
fn test_sphere_hit() {
    let center = Point3D::new(0.0, 0.0, 0.0);
    let sphere = Sphere::new(center, 1.0);
    let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(0.0, 0.0, 1.0));
    let hit = sphere.hit(&ray, 0.0, 10.0);
    assert_eq!(hit.unwrap().t, 1.0)
}
