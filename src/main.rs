use raytracer::{
    camera::Camera,
    render::render,
    vec3::{Point3D, Vec3},
    world::random_world,
};

fn main() {
    // Image settings
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: u32 = 50;

    // Instantiate world
    let world = random_world();

    // Instantiate Camera
    let viewfrom = Point3D::new(13.0, 2.0, 3.0);
    let viewat = Point3D::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f32 = 10.0;
    let aperture: f32 = 0.1;

    let camera = Camera::new(
        viewfrom,
        viewat,
        vup,
        20.0,
        ASPECT_RATIO,
        dist_to_focus,
        aperture,
    );

    // Render world
    render(
        IMAGE_HEIGHT,
        IMAGE_WIDTH,
        MAX_DEPTH,
        SAMPLES_PER_PIXEL,
        &world,
        camera,
    );
}
