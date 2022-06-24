use crate::cameras;
use crate::film;
use crate::helpers;
use crate::helpers::max_f64;
use crate::sampler::Sampler;
use crate::solver;
use crate::shader;
use crate::configuration;
use crate::light;
use std::{rc::Rc, cell::RefCell};

pub trait Renderer{
    fn render(&mut self);
    fn prepare_render(&mut self);
    fn save_image(&self, path: &str);
    fn evaluate(&mut self, t: f64);
}

pub struct CameraRayRenderer<C: cameras::Camera, F: film::Film> {
    camera: C,
    film: F
}

impl<C: cameras::Camera, F: film::Film> Renderer for CameraRayRenderer<C, F>{
    fn render(&mut self){
        for x in 0..configuration::width{
            for y in 0..configuration::height{
                let ray = self.camera.generate_ray(x as f64, y as f64, ((0.0,0.0),(0.0,0.0)));
                self.film.write_pixel(x, y, ray.0);
            }
        }
    }
    fn prepare_render(&mut self){

    }
    fn save_image(&self, path: &str){
        return self.film.save_image(path);
    }
    fn evaluate(&mut self, t: f64){
        self.camera.evaluate(t);
        self.film.evaluate(t);
    }
}

impl<C: cameras::Camera, F: film::Film> CameraRayRenderer<C, F>{
    pub fn new(camera: C, film: F) -> CameraRayRenderer<C, F>{
        CameraRayRenderer{
            camera: camera,
            film: film
        }
    }
}










pub struct SolverRenderer<C: cameras::Camera, F: film::Film, S: solver::Solver, H: shader::Shader, A: Sampler> {
    camera: C,
    film: F,
    solver: S,
    shader: H,
    sampler: A
}

fn generate_samples() -> [[f64;2];(configuration::samples*configuration::samples) as usize]{
    let mut elements = [[0.0, 0.0];(configuration::samples*configuration::samples) as usize];
    for sx in 0..configuration::samples{
        for sy in 0..configuration::samples{
            elements[(sx * configuration::samples + sy) as usize] = [sx as f64 / configuration::samples as f64, sy as f64 / configuration::samples as f64];
        }
    }
    return elements
}

impl<C: cameras::Camera, F: film::Film, S: solver::Solver, H: shader::Shader, A: Sampler> Renderer for SolverRenderer<C, F, S, H, A>{
    fn render(&mut self){
        if self.shader.is_lit() {
            panic!("Solver Renderer does not support lit shaders!");
        }

        let sample_inv = 1.0 / ((configuration::samples * configuration::samples) as f64);
        for x in 0..configuration::width{
            for y in 0..configuration::height{
                for s in self.sampler.generate_samples(x, y) {
                    let ray = self.camera.generate_ray(x as f64, y as f64, s);
                    // let ray = self.camera.generate_ray(x as f64, y as f64);
                    let i = self.solver.solve(ray);
                    if i.hit {
                        let col = self.shader.surface_props(x, y, &i);
                        self.film.write_pixel(x, y, [col.color[0] * sample_inv, col.color[1] * sample_inv, col.color[2] * sample_inv]);
                    }else{
                        let col = self.shader.miss_color(x, y, i);
                        self.film.write_pixel(x, y, [col[0] * sample_inv, col[1] * sample_inv, col[2] * sample_inv]);
                    }
                }
            }
        }
    }

    fn prepare_render(&mut self){
        self. film.prepare_render()
    }
    fn save_image(&self, path: &str){
        return self.film.save_image(path);
    }
    fn evaluate(&mut self, t: f64){
        self.camera.evaluate(t);
        self.film.evaluate(t);
        self.solver.evaluate(t);
        self.shader.evaluate(t);
    }
}

impl<C: cameras::Camera, F: film::Film, S: solver::Solver, H: shader::Shader, A: Sampler> SolverRenderer<C, F, S, H, A>{
    pub fn new(camera: C, film: F, solver: S, shader: H, sampler: A) -> SolverRenderer<C, F, S, H, A>{
        SolverRenderer{
            camera: camera,
            film: film,
            solver,
            shader,
            sampler
        }
    }
}










pub struct LightRenderer<C: cameras::Camera, F: film::Film, S: solver::Solver, H: shader::Shader, A: Sampler> {
    camera: C,
    film: F,
    solver: S,
    shader: H,
    sampler: A,
    lights: Vec<Box<dyn light::Light>>,
    ambient: [f64!(); 3]
}


impl<C: cameras::Camera, F: film::Film, S: solver::Solver, H: shader::Shader, A: Sampler> Renderer for LightRenderer<C, F, S, H, A>{
    fn render(&mut self){
        if !self.shader.is_lit() {
            panic!("Light Renderer does only support lit shaders!");
        }

        let sample_inv = 1.0 / ((configuration::samples * configuration::samples) as f64);
        for x in 0..configuration::width{
            for y in 0..configuration::height{
                for s in self.sampler.generate_samples(x, y) {
                    let ray = self.camera.generate_ray(x as f64, y as f64, s);
                    // let ray = self.camera.generate_ray(x as f64, y as f64);
                    let i = self.solver.solve(ray);
                    
                    if(!i.hit){
                        let col = self.shader.miss_color(x, y, i);
                        self.film.write_pixel(x, y, [col[0] * sample_inv, col[1] * sample_inv, col[2] * sample_inv]);
                    }else{
                        let surface = self.shader.surface_props(x, y, &i);
                        for l in &self.lights{
                            let light_info = l.illuminate(i.position);
                            let shadow_ray = (light_info.direction, [i.position[0] + i.normal[0] * configuration::light_min_dist, i.position[1] + i.normal[1] * configuration::light_min_dist, i.position[2] + i.normal[2] * configuration::light_min_dist]);
                            let i_simple = self.solver.solve_simple(shadow_ray, light_info.distance);
                            if !i_simple.hit {
                                let falloff = i.material.albedo * max_f64(0.0, vecmath::vec3_dot(i.normal, light_info.direction));
                                let diffuse = [light_info.light_intensity[0] * surface.color[0] * falloff * sample_inv * i.material.diffuse, light_info.light_intensity[1] * surface.color[1] * falloff * sample_inv * i.material.diffuse, light_info.light_intensity[2] * surface.color[2] * falloff * sample_inv * i.material.diffuse];

                                let light_dir_inv = [-light_info.direction[0], -light_info.direction[1], -light_info.direction[2]];

                                let r =  sample_inv * helpers::max_f64(helpers::dot_product(helpers::reflect(light_dir_inv, i.normal), light_info.direction), 0.0).powf(i.material.n_specular);
                                //println!("{:?}", r);
                                let specular = [light_info.light_intensity[0] * r * i.material.specular, light_info.light_intensity[1] * r * i.material.specular, light_info.light_intensity[2] * r * i.material.specular];
                                self.film.write_pixel(x, y, [diffuse[0] + specular[0], diffuse[1] + specular[1], diffuse[2] + specular[2]]);
                            }else{
                                self.film.write_pixel(x, y, [get_f64!(self.ambient[0]) * surface.color[0] * sample_inv, get_f64!(self.ambient[1]) * surface.color[1] * sample_inv, get_f64!(self.ambient[2]) * surface.color[2] * sample_inv]);
                            }
                        }
                    }
                }
            }
        }
    }

    fn prepare_render(&mut self){
        self. film.prepare_render()
    }
    fn save_image(&self, path: &str){
        return self.film.save_image(path);
    }
    fn evaluate(&mut self, t: f64){
        self.camera.evaluate(t);
        self.film.evaluate(t);
        self.solver.evaluate(t);
        self.shader.evaluate(t);
    }
}

impl<C: cameras::Camera, F: film::Film, S: solver::Solver, H: shader::Shader, A: Sampler> LightRenderer<C, F, S, H, A>{
    pub fn new(camera: C, film: F, solver: S, shader: H, sampler: A, lights: Vec<Box<dyn light::Light>>, ambient: [f64!(); 3]) -> LightRenderer<C, F, S, H, A>{
        LightRenderer{
            camera,
            film,
            solver,
            shader,
            sampler,
            lights,
            ambient
        }
    }
}