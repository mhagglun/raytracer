mod camera;
mod hit;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hit::{Hittable, World};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3D, Vec3};

/// Linearly blends the color depending on the height of the y-coordinate after scaling the ray
/// direction to unit length
fn ray_color(ray: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 {
        // Avoid inf recursion
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(ray, 0.001, f32::INFINITY) {
        let target = rec.point + Vec3::random_in_hemisphere(rec.normal);
        let ray = Ray::new(rec.point, target - rec.point);
        return 0.5 * ray_color(&ray, world, depth - 1);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const MAX_DEPTH: u32 = 4;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // World
    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Point3D::new(0.0, -100.5, -1.0),
        100.0,
    )));
    // Camera
    let camera = Camera::new();
    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = rand::thread_rng();
    for y in (0..IMAGE_HEIGHT).rev() {
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
