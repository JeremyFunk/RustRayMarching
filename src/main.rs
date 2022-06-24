#![allow(dead_code)]
#![allow(unused_variables)]

use std::borrow::{BorrowMut, Borrow};
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
type Sample = (f64,f64);
// FilmSample, LensSample
type CameraSample = (Sample,Sample);
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
mod scene;
mod light;
use primitive::Material;
use renderers::Renderer;
use scene::SceneObject;
use std::time::{Duration, Instant};


fn main() {
    //println!("{:?}", helpers::mat_transformation([0.0, 0.0, 0.0], [90.0, 0.0, 0.0], [2.0, 3.0, 4.0]));
    //return;
    if configuration::render_scene {
        render_scene();
    } else {
        render_code();
    }
}

fn add_prim(so: &SceneObject) -> Box<dyn primitive::Primitive>{
    let mut m = Material::new_base();
    m.albedo = f64!(1.0);
    m.specular = f64!(0.8);
    m.diffuse = f64!(0.2);
    m.n_specular = f64!(10.0);

    if so.prim_type == 1 {
        let pos_modifier = Vec::<Box<dyn modifier::PosModifier>>::new();
        return Box::new(primitive::Sphere::new(so.values[0].to_owned(), [so.position[0].to_owned(), so.position[1].to_owned(), so.position[2].to_owned()], [so.rotation[0].to_owned(), so.rotation[1].to_owned(), so.rotation[2].to_owned()], [so.scale[0].to_owned(), so.scale[1].to_owned(), so.scale[2].to_owned()], pos_modifier,  Some(m)));
    }
    if so.prim_type == 2 {
        let pos_modifier = Vec::<Box<dyn modifier::PosModifier>>::new();
        return Box::new(primitive::Torus::new(so.values[0].to_owned(), so.values[1].to_owned(), [so.position[0].to_owned(), so.position[1].to_owned(), so.position[2].to_owned()], [so.rotation[0].to_owned(), so.rotation[1].to_owned(), so.rotation[2].to_owned()], [so.scale[0].to_owned(), so.scale[1].to_owned(), so.scale[2].to_owned()], pos_modifier, Some(m)));
    }
    if so.prim_type == 3 {
        let pos_modifier = Vec::<Box<dyn modifier::PosModifier>>::new();
        return Box::new(primitive::Cube::new([so.values[0].to_owned(), so.values[1].to_owned(), so.values[2].to_owned()], [so.position[0].to_owned(), so.position[1].to_owned(), so.position[2].to_owned()], [so.rotation[0].to_owned(), so.rotation[1].to_owned(), so.rotation[2].to_owned()], [so.scale[0].to_owned(), so.scale[1].to_owned(), so.scale[2].to_owned()], pos_modifier, Some(m)));
    }
    else {
        let pos_modifier = Vec::<Box<dyn modifier::PosModifier>>::new();
        return Box::new(primitive::Mandelbulb::new(so.values[0].to_owned(), [so.position[0].to_owned(), so.position[1].to_owned(), so.position[2].to_owned()], [so.rotation[0].to_owned(), so.rotation[1].to_owned(), so.rotation[2].to_owned()], [so.scale[0].to_owned(), so.scale[1].to_owned(), so.scale[2].to_owned()], pos_modifier));
    }
}

fn render_scene(){
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
            let handle: JoinHandle<()>;

            if configuration::render_scene { 
                handle = thread::spawn(|| {
                    render_frames(current_frame, "");
                });
            }else{
                handle = thread::spawn(|| {
                    render_frames_code(current_frame, "");
                });
            }

            handles.push(handle);
        }

        for h in handles{
            h.join().unwrap();
        }
    }else {
        let start = SystemTime::now();
        if configuration::render_scene { 
            render_frames(vec!(configuration::still_frame), "result");
        }else{
            render_frames_code(vec!(configuration::still_frame), "result");
        }
        
        match start.elapsed() {
            Ok(start) => {
                println!("Total in {}", start.as_millis() as f64 * 0.001);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

fn render_frames(frames: Vec<u32>, file_name: &str){
    let scene = scene::load_scene("test.rma.json");
    let mut primitives = Vec::<Box<dyn primitive::Primitive>>::new();
    
    let mut group_modifier_indices: Vec<i32> = Vec::new();

    for go in scene.group_modifiers{
        if go.prim0 != -1 && go.prim1 != -1 {
            let mut cur_prims: Vec<Box<dyn primitive::Primitive>> = Vec::new();
            group_modifier_indices.push(go.prim0);
            cur_prims.push(add_prim(&scene.objects[go.prim0 as usize]));
            group_modifier_indices.push(go.prim1);
            cur_prims.push(add_prim(&scene.objects[go.prim1 as usize]));
            if go.prim2 != -1 {
                group_modifier_indices.push(go.prim2);
                cur_prims.push(add_prim(&scene.objects[go.prim2 as usize]));
            }
            if go.prim3 != -1 {
                group_modifier_indices.push(go.prim3);
                cur_prims.push(add_prim(&scene.objects[go.prim3 as usize]));
            }
            if go.modifier == 1 {
                primitives.push(Box::new(primitive::GroupPrimitive::new_union(cur_prims)));
            }
            else if go.modifier == 2 {
                primitives.push(Box::new(primitive::GroupPrimitive::new_subtraction(cur_prims)));
            }
            else if go.modifier == 3 {
                primitives.push(Box::new(primitive::GroupPrimitive::new_intersection(cur_prims)));
            }
            else if go.modifier == 4 {
                primitives.push(Box::new(primitive::GroupPrimitive::new_union_smooth(cur_prims, go.prim_attribute)));
            }
            else if go.modifier == 5 {
                primitives.push(Box::new(primitive::GroupPrimitive::new_subtraction_smooth(cur_prims, go.prim_attribute)));
            }
            else if go.modifier == 6 {
                primitives.push(Box::new(primitive::GroupPrimitive::new_intersection_smooth(cur_prims, go.prim_attribute)));
            }
        }
    }
    for so in 0..scene.objects.len() {
        let mut found = false;
        for i in 0..group_modifier_indices.len(){
            if group_modifier_indices[i] == so as i32{
                found = true;
                break;
            }
        }
        if !found {
            let pos_modifier = Vec::<Box<dyn modifier::PosModifier>>::new();
            primitives.push(add_prim(&scene.objects[so]));
        }
    }
    
    let camera = cameras:: PinholeCamera::new(
        [scene.camera.cam_pos[0].to_owned(), scene.camera.cam_pos[1].to_owned(), scene.camera.cam_pos[2].to_owned()],
        [scene.camera.cam_py[0].to_owned(), scene.camera.cam_py[1].to_owned(), f64!(0.0)]
    );
    let film = film::BasicFilm::new(vec![], vec![]);
    let solver = solver::GeneralSolver::new(primitives);
    let bg_shader = shader::BackgroundLinearYGradient::new([0.05, 0.02, 0.04], [0.1, 0.06, 0.06]);
    //let shader = shader::NormalShader::new(Box::new(bg_shader));
    let shader = shader::PhongShader::new(Box::new(bg_shader));
    //let shader = shader::FractalShader::new(f64v!([0.1, 0.1, 0.4]), f64v!([0.2, 0.9, 0.8]), f64!(30.0), [f64!(0.0), f64!(-45.0), f64!(-45.0)], Box::new(bg_shader));
    let sampler = sampler::JitterSampler::new(0.5);
    let mut lights = Vec::<Box<dyn light::Light>>::new();
    lights.push(Box::new(light::DirectionalLight::new(f64v!(vecmath::vec3_normalized([-0.5, 0.3, -1.0])), f64v!([1.0, 0.2, 0.2]), f64!(1.5))));
    lights.push(Box::new(light::PointLight::new(f64v!([0.0, 2.2, 1.0]), f64v!([0.2, 1.0, 0.2]), f64!(100.0))));

    let mut renderer= renderers::LightRenderer::new(camera, film, solver, shader, sampler, lights, f64v!([0.01, 0.01, 0.01]));
    //let mut renderer= renderers::SolverRenderer::new(camera, film, solver, shader, sampler);
    //let mut renderer= renderers::CameraRayRenderer::new(camera, film);

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

fn render_code(){
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
                render_frames_code(current_frame, "");
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
}

fn render_frames_code(frames: Vec<u32>, file_name: &str){
    let camera_pos_z_ref = f64!(2.6);
    evaluator::InterpolatorEvaluator::new(2.6, 2.2, 3.0, true, Box::new(transition::Smoothstep::new(2.0)), camera_pos_z_ref.clone());
    let camera = cameras:: PinholeCamera::new(
        // f64!(0.005),
        // f64!(2.2),
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
    evaluator::InterpolatorEvaluator::new(0.35, 0.3, 3.0, true, Box::new(transition::Smoothstep::new(2.0)), bloom_cut_ref.clone());
    let bloom_factor_ref = f64!(1.0);
    evaluator::InterpolatorEvaluator::new(1.0, 2.0, 3.0, true, Box::new(transition::Smoothstep::new(2.0)), bloom_factor_ref.clone());
    let bloom_size_ref = f64!(3.0);
    evaluator::InterpolatorEvaluator::new(3.0, 6.0, 3.0, true, Box::new(transition::Smoothstep::new(2.0)), bloom_size_ref.clone());

    let bloom = postprocessor::BloomPostProcessor::new(bloom_cut_ref, bloom_factor_ref, bloom_size_ref);

    let film = film::BasicFilm::new(vec![Box::new(color_filter), Box::new(gray_filter)], vec![Box::new(bloom), Box::new(noise)]);
    let solver = solver::GeneralSolver::new(primitives);
    let bg_shader = shader::BackgroundLinearYGradient::new([0.05, 0.02, 0.04], [0.1, 0.06, 0.06]);
    // let shader = shader::NormalShader::new(Box::new(bg_shader));

    
    let shader_rot_z = f64!(-45.0);
    evaluator::InterpolatorEvaluator::new(-45.0, 90.0, 3.0, true, Box::new(transition::Linear::new()),shader_rot_z.clone());
    let shader = shader::FractalShader::new(f64v!([0.1, 0.1, 0.4]), f64v!([0.2, 0.9, 0.8]), f64!(30.0), [shader_rot_z, f64!(-45.0), f64!(-45.0)], Box::new(bg_shader));
    let sampler = sampler::JitterSampler::new(0.5);
    //let mut renderer= renderers::CameraRayRenderer::new(camera, film);
    let mut renderer= renderers::SolverRenderer::new(camera, film, solver, shader, sampler);
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