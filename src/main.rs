use image::{RgbImage, ImageBuffer, Rgb};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {
    let mut buffer: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let r = x as f32 / (WIDTH-1) as f32;
        let g = y as f32 / (HEIGHT-1) as f32;
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done."),
    };
}
