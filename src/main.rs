mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::{
    io::{stderr, Write},
    rc::Rc,
};

use camera::Camera;
use hit::{Hittable, World};
use material::{Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3D};

use crate::{material::Dielectric, vec3::Vec3};

/// Linearly blends the color depending on the height of the y-coordinate after scaling the ray
/// direction to unit length
fn ray_color(ray: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 {
        // Avoid inf recursion
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(ray, 0.001, f32::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mtrl.scatter(ray, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const MAX_DEPTH: u32 = 50;
    const SAMPLES_PER_PIXEL: u32 = 300;

    // World
    let mut world = World::new();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_right = Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.add(Box::new(sphere_ground));
    world.add(Box::new(sphere_center));
    world.add(Box::new(sphere_left));
    world.add(Box::new(sphere_right));

    // Camera
    let camera = Camera::new(
        Point3D::new(-2.0, 2.0, 1.0),
        Point3D::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        ASPECT_RATIO,
    );
    // Start Rendering
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = rand::thread_rng();
    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!(
            "\rRendering image. Columns completed / total: {:3} / {}",
            IMAGE_HEIGHT - y - 1,
            IMAGE_HEIGHT
        );
        stderr().flush().unwrap();

        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let rndm_x: f32 = rng.gen();
                let rndm_y: f32 = rng.gen();

                let u = ((x as f32) + rndm_x) / ((IMAGE_WIDTH - 1) as f32);
                let v = ((y as f32) + rndm_y) / ((IMAGE_HEIGHT - 1) as f32);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
}
