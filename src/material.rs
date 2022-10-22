use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().unit_vector();
        if scatter_direction.near_zero() {
            // If random unit vector happens to be the exact opposite of the normal, then we get a
            // zero vector
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}

/// Materials

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(c: Color) -> Metal {
        Metal { albedo: c }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction.reflect(rec.normal).unit_vector();
        let scattered = Ray::new(rec.point, reflected);

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
