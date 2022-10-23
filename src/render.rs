use std::io::{stderr, Write};

use rand::Rng;
use rayon::prelude::*;

use crate::{
    camera::Camera,
    materials::Scatter,
    ray::{Hittable, Ray},
    vec3::Color,
    world::World,
};

/// Linearly blends the color depending on ray direction and the index of refraction for the
/// material hit
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

pub fn render(
    image_height: u32,
    image_width: u32,
    max_depth: u32,
    samples_per_pixel: u32,
    world: &World,
    camera: Camera,
) {
    //Start rendering
    println!("P3\n{} {}\n255", image_width, image_height);
    for y in (0..image_height).rev() {
        eprint!(
            "\rRendering progress: {:2}%",
            100 * (image_height - y) / image_height,
        );
        stderr().flush().unwrap();

        let scanline: Vec<Color> = (0..image_width)
            .into_par_iter()
            .map(|x| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let mut rng = rand::thread_rng();
                    let random_u: f32 = rng.gen();
                    let random_v: f32 = rng.gen();

                    let u = ((x as f32) + random_u) / ((image_width - 1) as f32);
                    let v = ((y as f32) + random_v) / ((image_height - 1) as f32);

                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, world, max_depth);
                }

                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(samples_per_pixel));
        }
    }
}
