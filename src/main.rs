mod ray;
mod vec3;

use image::{RgbImage, ImageBuffer, Rgb};
use ray::Ray;
use vec3::{Color, Vec3};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

/// Linearly blends the color depending on the height of the y-coordinate after scaling the ray
/// direction to unit length
fn ray_color(r: Ray) -> Color {
    let unit_direction = r.direction.unit_vector();  
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}


fn main() {
    
    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
   
    let origin = Vec3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - 0.5 * horizontal  - 0.5 * vertical - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut buffer: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let u = x as f32 / (WIDTH-1) as f32;
        let v = y as f32 / (HEIGHT-1) as f32;

        let r = Ray::new(origin, lower_left_corner + u * horizontal + v*vertical - origin);
        let c = ray_color(r);
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
