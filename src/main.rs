mod hit;
mod ray;
mod sphere;
mod vec3;

use hit::{Hittable, World};
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3D, Vec3};

/// Linearly blends the color depending on the height of the y-coordinate after scaling the ray
/// direction to unit length
fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f32::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 1.0;
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // World
    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Point3D::new(0.0, -100.5, -1.0),
        100.0,
    )));
    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - 0.5 * horizontal - 0.5 * vertical - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let u = x as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = y as f32 / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let c = ray_color(&r, &world);

            let ir = (255.999 * c.x.sqrt()) as u8;
            let ig = (255.999 * c.y.sqrt()) as u8;
            let ib = (255.999 * c.z.sqrt()) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
