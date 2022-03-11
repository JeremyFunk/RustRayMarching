#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread::{JoinHandle, self};
use std::{time::SystemTime};
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
mod transition;
mod evaluator;
mod modifier;
mod primitive;
mod sampler;
mod cameras;
mod filter;
mod postprocessor;
mod film;
mod solver;
mod shader;
mod renderers;
use renderers::Renderer;


fn main() {

    if configuration::video {
        let mut frames: Vec<Vec<u32>> = Vec::new();
        for _t in 0..configuration::threads{
            frames.push(Vec::new())
        }

        for i in configuration::start_frame..configuration::end_frame{
            frames[(i % configuration::threads) as usize].push(i);
        }
        
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for current_frame in frames{
            let handle = thread::spawn(|| {
                render_frames(current_frame, "");
            });
            handles.push(handle);
        }

        for h in handles{
            h.join().unwrap();
        }
    }else {
        let start = SystemTime::now();
        render_frames(vec!(configuration::still_frame), "result");
        match start.elapsed() {
            Ok(start) => {
                println!("Total in {}", start.as_millis() as f64 * 0.001);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
    

    //     // Construct a new RGB ImageBuffer with the specified width and height.
    // let mut img: RgbImage = ImageBuffer::new(512, 512);

    // for (x, y, pixel) in img.enumerate_pixels_mut() {
    //     let r = (0.3 * x as f32) as u8;
    //     let b = (0.3 * y as f32) as u8;
    //     *pixel = image::Rgb([r, 0, b]);
    // }
}

fn render_frames(frames: Vec<u32>, file_name: &str){
    let camera_pos_z_ref = f64!(2.5);
    evaluator::InterpolatorEvaluator::new(2.5, 2.0, 3.0, true, Box::new(transition::Smoothstep::new(2.0)), camera_pos_z_ref.clone());
    let camera = cameras::PinholeCamera::new(
        [f64!(0.0), f64!(0.0), camera_pos_z_ref],
        f64v!([0.0, 0.0, 0.0])
    );
    let pos_modifier = Vec::<Box<dyn modifier::PosModifier>>::new();
    // pos_modifier.push(Box::new(modifier::Distort::new(1.1, [0.0,0.0,0.0], 2.2)));
    let mut primitives = Vec::<Box<dyn primitive::Primitive>>::new();

    let mandel_power_ref = f64!(9.0);
    evaluator::InterpolatorEvaluator::new(0.6, 15.0, 5.5, true, Box::new(transition::Smoothstep::new(1.0)), mandel_power_ref.clone());

    let mandel_rot_x_ref = f64!(9.0);
    evaluator::InterpolatorEvaluator::new(160.0, 180.0, 2.75, true, Box::new(transition::Smoothstep::new(2.0)), mandel_rot_x_ref.clone());

    let mandel_scale_ref = f64!(0.3);
    evaluator::CombineEvaluator::new(vec![
        evaluator::CombineEvaluatorInfo::new(0.0, 4.5, Box::new(evaluator::InterpolatorEvaluator::new_get(1.0, 1.25, 4.5, true, Box::new(transition::Smoothstep::new(2.0))))),
        evaluator::CombineEvaluatorInfo::new(4.5, 6.5, Box::new(evaluator::InterpolatorEvaluator::new_get(1.25, 0.25, 2.5, true, Box::new(transition::Smoothstep::new(1.0)))))
    ], mandel_scale_ref.clone());

    let mandel = primitive::Mandelbulb::new(mandel_power_ref, f64v!([0.0, 0.0, 0.0]), [mandel_rot_x_ref, f64!(0.0), f64!(0.0)], [mandel_scale_ref.clone(), mandel_scale_ref.clone(), mandel_scale_ref], pos_modifier);
    primitives.push(
        // Box::new(primitive::Box::new([1.0,1.0,1.0], [0.0, 0.0, -10.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]))
        // Box::new(primitive::Sphere::new(4.0, [0.0, 0.0, -10.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], pos_modifier))
        Box::new(mandel)
    );

    let color_filter_ref = f64!(0.3);
    evaluator::CombineEvaluator::new(vec![
        evaluator::CombineEvaluatorInfo::new(0.0, 4.5, Box::new(evaluator::FloatEvaluator::new_get(1.0))),
        evaluator::CombineEvaluatorInfo::new(4.5, 6.0, Box::new(evaluator::InterpolatorEvaluator::new_get(1.0, 0.0, 1.5, true, Box::new(transition::Smoothstep::new(1.0)))))
    ], color_filter_ref.clone());
    let color_filter = filter::ColorShift::new([color_filter_ref.clone(), color_filter_ref.clone(), color_filter_ref], filter::ColorShiftMode::Mul);
    
    let gray_filter_ref = f64!(0.3);
    evaluator::CombineEvaluator::new(vec![
        evaluator::CombineEvaluatorInfo::new(0.0, 3.0, Box::new(evaluator::FloatEvaluator::new_get(0.0))),
        evaluator::CombineEvaluatorInfo::new(3.0, 6.0, Box::new(evaluator::InterpolatorEvaluator::new_get(0.0, 0.8, 3.0, true, Box::new(transition::Smoothstep::new(2.0)))))
    ], gray_filter_ref.clone());
    let gray_filter = filter::GrayFilter::new(gray_filter_ref);
    
    let noise = postprocessor::NoisePostProcessor::new(f64!(0.01), f64!(0.005));
    
    let bloom_cut_ref = f64!(0.3);
    evaluator::InterpolatorEvaluator::new(0.4, 0.35, 3.0, true, Box::new(transition::Smoothstep::new(2.0)), bloom_cut_ref.clone());
    let bloom_factor_ref = f64!(1.0);
    evaluator::InterpolatorEvaluator::new(0.5, 1.0, 3.0, true, Box::new(transition::Smoothstep::new(2.0)), bloom_factor_ref.clone());
    let bloom_size_ref = f64!(3.0);
    evaluator::InterpolatorEvaluator::new(3.0, 6.0, 3.0, true, Box::new(transition::Smoothstep::new(2.0)), bloom_size_ref.clone());

    let film = film::BasicFilm::new(vec![Box::new(color_filter), Box::new(gray_filter)], vec![Box::new(noise), Box::new(postprocessor::BloomPostProcessor::new(bloom_cut_ref, bloom_factor_ref, bloom_size_ref))]);
    let solver = solver::GeneralSolver::new(primitives);
    // let shader = shader::NormalShader::new();

    let bg_shader = shader::BackgroundLinearYGradient::new([0.05, 0.02, 0.04], [0.1, 0.06, 0.06]);
    let shader_rot_z = f64!(-45.0);
    evaluator::InterpolatorEvaluator::new(-45.0, 90.0, 3.0, true, Box::new(transition::Linear::new()),shader_rot_z.clone());
    let shader = shader::FractalShader::new(f64v!([0.4, 0.1, 0.2]), f64v!([0.9, 0.2, 0.3]), f64!(30.0), [shader_rot_z, f64!(-45.0), f64!(-45.0)], Box::new(bg_shader));
    // let mut renderer= renderers::CameraRayRenderer::new(camera, film);
    let mut renderer= renderers::SolverRenderer::new(camera, film, solver, shader);

    for i in frames{
        let t = i as f64 / configuration::ups;

        evaluator::evaluate(t);
        renderer.prepare_render();
        renderer.evaluate(t);
        renderer.render();

        let path = if file_name == "" {format!("results/{}.png", i)} else{format!("results/{}.png", file_name)};

        renderer.save_image(&path);
    }
}