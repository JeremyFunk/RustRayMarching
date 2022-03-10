
use std::{io, time::SystemTime};
use std::{rc::Rc, cell::RefCell};

#[macro_export]
macro_rules! f64 {
    () => {
        Rc<RefCell<f64>>
    };
    ($x:expr) => {
        Rc::new(RefCell::new($x))
    };
}

#[macro_export]
macro_rules! get_f64 {
    ($x:expr) => {
        *$x.borrow()
    };
}
#[macro_export]
macro_rules! get_ref_f64 {
    ($x:expr) => {
        *$x.as_ref().borrow()
    };
}
#[macro_export]
macro_rules! get_f64v {
    ($x:expr) => {
        [*$x[0].borrow(), *$x[1].borrow(), *$x[2].borrow()]
    };
}
#[macro_export]
macro_rules! get_ref_f64v {
    ($x:expr) => {
        [*$x[0].as_ref().borrow(), *$x[1].as_ref().borrow(), *$x[2].as_ref().borrow()]
    };
}
macro_rules! f64v {
    () => {
        [Rc<RefCell<f64>>;3]
    };
    ($x:expr) => {
        [Rc::new(RefCell::new($x[0])),Rc::new(RefCell::new($x[1])),Rc::new(RefCell::new($x[2]))]
    };
}

// #[macro_export]
// macro_rules! eval_v {
//     ($x:expr) => {
//         evaluator::convert_to_evaluator($x)
//     };
// }

// #[macro_export]
// macro_rules! eval_t {
//     () => {
//         Box<dyn evaluator::Evaluator>
//     };
// }

// #[macro_export]
// macro_rules! eval_t_v {
//     () => {
//         [Box<dyn evaluator::Evaluator>;3]
//     };
// }
// #[macro_export]
// macro_rules! evaluator_evs {
//     (
//         $name:ident {
//         $($field_name:ident,)*}
//     ) => {
//         impl $name {
//             $(pub fn $field_name(&mut self, val: eval_t!()){self.$field_name = val;})*
//         }
//     }
// }

// #[macro_export]
// macro_rules! evaluator_struct {
//     (
//         $name:ident {
//         $($default_field_name:ident: $default_field_type:ty,)*},
//         {$($evaluator_name:ident,)*}
//     ) => {
//         struct $name {
//             $($default_field_name: $default_field_type,)*
//             $($evaluator_name: Box::new(evaluator::FloatEvaluator::new($field_type_2)),)*
//             // $($float_name: f64,)*
//         }
//     }
// }


// macro_rules! eval_new {
//     (
//         $name:ident {
//         $($default_field_name:ident: $default_field_type:expr,)*},
//         {$($evaluator_name:ident: $field_type_2:expr,)*}
//     ) => {
//         $name {
//             $($default_field_name: $default_field_type,)*
//             $($evaluator_name: Box::new(evaluator::FloatEvaluator::new($field_type_2)),)*
//             // $($float_name: f64,)*
//         }
//     }
// }
// macro_rules! eval_func {
//     (   
//         {$($default_field_name:ident:$default_ev_name:ident,)*}
//     ) => {
//         fn evaluate(&mut self, t: f64){
//             $(self.$default_field_name = self.$default_ev_name.evaluate(t);)*
//         }
//     }
// }
// macro_rules! _eval_func {
//     (   
//         {$($default_field_name:ident:$default_ev_name:ident,)*}
//     ) => {
//         fn _evaluate(&mut self, t: f64){
//             $(self.$default_field_name = self.$default_ev_name.evaluate(t);)*
//         }
//     }
// }
// D, O
type Ray = ([f64;3],[f64;3]);
mod configuration;
// mod evaluator
mod helpers;
mod evaluator;
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

    let camera_pos_z_ref = f64!(2.5);
    evaluator::InterpolatorEvaluator::new(2.5, 20.0, 2.0, true, camera_pos_z_ref.clone());
    let camera = cameras::PinholeCamera::new(
        [f64!(0.0), f64!(0.0), camera_pos_z_ref],
        f64v!([0.0, 0.0, 0.0])
    );
    let mut pos_modifier = Vec::<Box<dyn modifier::PosModifier>>::new();
    // pos_modifier.push(Box::new(modifier::Distort::new(1.1, [0.0,0.0,0.0], 2.2)));
    let mut primitives = Vec::<Box<dyn primitive::Primitive>>::new();

    let power_ref = f64!(9.0);

    evaluator::InterpolatorEvaluator::new(9.0, 20.0, 2.0, true, power_ref.clone());
    let mandel = primitive::Mandelbulb::new(power_ref,f64v!([0.0, 0.0, 0.0]), f64v!([35.0, 8.0, 14.0]), f64v!([1.0, 1.0, 1.0]), pos_modifier);
    primitives.push(
        // Box::new(primitive::Box::new([1.0,1.0,1.0], [0.0, 0.0, -10.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]))
        // Box::new(primitive::Sphere::new(4.0, [0.0, 0.0, -10.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], pos_modifier))
        Box::new(mandel)
    );

    let filter = filter::ColorShift::new(f64v!([1.0, 1.0, 1.0]), filter::ColorShiftMode::Mul);
    let film = film::BasicFilm::new(filter);
    let solver = solver::GeneralSolver::new(primitives);
    // let shader = shader::NormalShader::new();
    let shader = shader::FractalShader::new([0.4, 0.1, 0.2], [0.9, 0.2, 0.3], 30.0, [-45.0, -45.0, -45.0]);
    // let mut renderer= renderers::CameraRayRenderer::new(camera, film);
    let mut renderer= renderers::SolverRenderer::new(camera, film, solver, shader);

    if configuration::Config.video {
        for i in 0..240{
            let t = i as f64 / 24.0;

            evaluator::evaluate(t);
            renderer.prepare_render();
            renderer.evaluate(t);
            renderer.render();

            let img = renderer.get_image();
            img.save(format!("{}.png", i)).unwrap();
        }
    }else {
        let start = SystemTime::now();
        for i in 0..20{
            let frame_start = SystemTime::now();

            evaluator::evaluate(0.0);
            renderer.prepare_render();
            renderer.evaluate(0.0);
            renderer.render();

            let img = renderer.get_image();
            img.save("result.png").unwrap();
            match(frame_start.elapsed()){
                Ok(frame_start) => {
                    println!("Frame {} in {}", i, frame_start.as_millis() as f64 * 0.001);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        match(start.elapsed()){
            Ok(start) => {
                println!("Total in {}", start.as_millis() as f64 * 0.001);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
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

