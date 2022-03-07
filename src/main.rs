use std::io;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Rgb};

// D, O
type Ray = ([f64;3],[f64;3]);

mod configuration;
mod helpers;
use configuration::Config;
mod primitive;
mod cameras;
mod film;
mod solver;
mod shader;
mod renderers;
use renderers::Renderer;


fn main() {
    let camera = cameras::PinholeCamera::new(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0]
    );

    let mut primitives = Vec::<Box<dyn primitive::Primitive>>::new();
    primitives.push(
        // Box::new(primitive::Box::new([1.0,1.0,1.0], [0.0, 0.0, -10.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]))
        Box::new(primitive::Sphere::new(1.0, [0.0, 0.0, -10.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]))
    );

    let film = film::BasicFilm::new();
    let solver = solver::GeneralSolver::new(primitives);
    let shader = shader::NormalShader::new();
    // let mut renderer= renderers::CameraRayRenderer::new(camera, film);
    let mut renderer= renderers::SolverRenderer::new(camera, film, solver, shader);



    renderer.render();
    let img = renderer.get_image();

    // println!("Hello, world!");
    
    //     // Construct a new RGB ImageBuffer with the specified width and height.
    // let mut img: RgbImage = ImageBuffer::new(512, 512);

    // for (x, y, pixel) in img.enumerate_pixels_mut() {
    //     let r = (0.3 * x as f32) as u8;
    //     let b = (0.3 * y as f32) as u8;
    //     *pixel = image::Rgb([r, 0, b]);
    // }
    img.save("test.png").unwrap();
}

