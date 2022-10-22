use crate::{
    materials::Material,
    ray::{HitRecord, Hittable, Ray},
    vec3::Point3D,
};

pub struct Sphere {
    pub center: Point3D,
    pub radius: f32,
    pub mtrl: Material,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f32, mtrl: Material) -> Sphere {
        Sphere {
            center,
            radius,
            mtrl,
        }
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
                    let outward_facing = ray.direction.dot(normal) < 0.0;

                    return Some(HitRecord {
                        point: p,
                        normal: if outward_facing { normal } else { -normal },
                        outward_facing,
                        t: *root,
                        mtrl: &self.mtrl,
                    });
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::materials::{Lambertian, Material};
    use crate::vec3::Color;

    use super::Hittable;
    use super::{Point3D, Ray, Sphere};

    #[test]
    fn test_sphere_hit() {
        let sphere = {
            let center = Point3D::new(0.0, 0.0, 0.0);
            let mtrl = Material::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
            Sphere {
                center,
                radius: 1.0,
                mtrl,
            }
        };
        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(0.0, 0.0, 1.0));
        let hit = sphere.hit(&ray, 0.0, 10.0);
        assert_eq!(hit.unwrap().t, 1.0)
    }
}
