mod ray;
mod sphere;
mod vec3;

use image::{ImageBuffer, Rgb, RgbImage};
use ray::{ray_color, Ray};
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    // Camera
    let aspect_ratio = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - 0.5 * horizontal - 0.5 * vertical - Vec3::new(0.0, 0.0, focal_length);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    let image_width: u32 = 400;
    let image_height: u32 = image_width / aspect_ratio as u32;

    // Render
    let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let u = x as f32 / (image_width - 1) as f32;
        let v = y as f32 / (image_height - 1) as f32;

        let r = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );
        let c = ray_color(sphere, r);
        let ir = (255.999 * c.x.sqrt()) as u8;
        let ig = (255.999 * c.y.sqrt()) as u8;
        let ib = (255.999 * c.z.sqrt()) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done."),
    };
}
