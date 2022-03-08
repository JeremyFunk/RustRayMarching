use std::io;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Rgb};


#[macro_export]
macro_rules! eval {
    ( $x:expr ) => {
        Box::new(evaluator::FloatEvaluator::new($x))
    };
}

#[macro_export]
macro_rules! eval_b {
    () => {
        Box<dyn evaluator::Evaluator>
    };
}

#[macro_export]
macro_rules! evaluator_struct {
    (
        $name:ident {
        $($field_name:ident,)*}
    ) => {
        impl $name {
            $(pub fn $field_name(&mut self, val: eval_b!()){self.$field_name = val;})*
        }
    }
}


macro_rules! eval_new {
    (
        $name:ident {
        $($default_field_name:ident: $default_field_type:expr,)*},
        {$($evaluator_name:ident: $field_type_2:expr,)*}
    ) => {
        $name {
            $($default_field_name: $default_field_type,)*
            $($evaluator_name: Box::new(evaluator::FloatEvaluator::new($field_type_2)),)*
            // $($float_name: f64,)*
        }
    }
}

macro_rules! eval_func {
    (   
        {$($default_field_name:ident:$default_ev_name:ident,)*}
    ) => {
        fn _evaluate(&mut self, t: f64){
            $(self.$default_field_name = self.$default_ev_name.evaluate(t);)*
        }
    }
}
// D, O
type Ray = ([f64;3],[f64;3]);
mod configuration;
mod evaluator;
mod helpers;
mod modifier;
mod primitive;
mod cameras;
mod filter;
mod film;
mod solver;
mod shader;
mod renderers;
use renderers::Renderer;


fn main() {
    let camera = cameras::PinholeCamera::new(
        [0.0, 0.0, 2.5],
        [0.0, 0.0, 0.0]
    );
    let mut pos_modifier = Vec::<Box<dyn modifier::PosModifier>>::new();
    // pos_modifier.push(Box::new(modifier::Distort::new(1.1, [0.0,0.0,0.0], 2.2)));
    let mut primitives = Vec::<Box<dyn primitive::Primitive>>::new();
    let mut mandel = primitive::Mandelbulb::new(5.0, [0.0, 0.0, 0.0], [35.0, 8.0, 14.0], [1.0, 1.0, 1.0], pos_modifier);
    mandel.power_ev(Box::new(evaluator::InterpolatorEvaluator::new(0.0, 9.0, 10.0, true)));
    primitives.push(
        // Box::new(primitive::Box::new([1.0,1.0,1.0], [0.0, 0.0, -10.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]))
        // Box::new(primitive::Sphere::new(4.0, [0.0, 0.0, -10.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], pos_modifier))
        Box::new(mandel)
    );

    let filter = filter::ColorShift::new([1.0, 1.0, 1.0], filter::ColorShiftMode::Mul);
    let film = film::BasicFilm::new(filter);
    let solver = solver::GeneralSolver::new(primitives);
    // let shader = shader::NormalShader::new();
    let shader = shader::FractalShader::new([0.4, 0.1, 0.2], [0.9, 0.2, 0.3], 30.0, [-45.0, -45.0, -45.0]);
    // let mut renderer= renderers::CameraRayRenderer::new(camera, film);
    let mut renderer= renderers::SolverRenderer::new(camera, film, solver, shader);


    for i in 0..240{
        let t = i as f64 / 24.0;
        renderer.prepare_render();
        renderer.evaluate(t);
        renderer.render();
        let img = renderer.get_image();
        img.save(format!("{}.png", i)).unwrap();
    }

    

    // println!("Hello, world!");
    
    //     // Construct a new RGB ImageBuffer with the specified width and height.
    // let mut img: RgbImage = ImageBuffer::new(512, 512);

    // for (x, y, pixel) in img.enumerate_pixels_mut() {
    //     let r = (0.3 * x as f32) as u8;
    //     let b = (0.3 * y as f32) as u8;
    //     *pixel = image::Rgb([r, 0, b]);
    // }
}

