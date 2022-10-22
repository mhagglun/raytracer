use rand::Rng;

use crate::materials::Material;
use crate::materials::{Dielectric, Lambertian, Metal};
use crate::ray::Hittable;
use crate::ray::{HitRecord, Ray};
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3D};

#[derive(Default)]
pub struct World {
    pub items: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn add(&mut self, item: Box<dyn Hittable>) {
        self.items.push(item);
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for item in &self.items {
            if let Some(rec) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}

pub fn random_world() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_sphere = Sphere::new(
        Point3D::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    );

    world.add(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f32 = rng.gen();
            let center = Point3D::new(
                (a as f32) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f32) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                let sphere_mat = Material::Lambertian(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4, 1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Material::Metal(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Material::Dielectric(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(Box::new(sphere));
            }
        }
    }

    let mat1 = Material::Dielectric(Dielectric::new(1.5));
    let mat2 = Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3D::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3D::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3D::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere3));

    world
}
