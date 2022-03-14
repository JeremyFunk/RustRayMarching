use crate::cameras;
use crate::film;
use crate::sampler::Sampler;
use crate::solver;
use crate::shader;
use crate::configuration;

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
        let sample_inv = 1.0 / ((configuration::samples * configuration::samples) as f64);
        for x in 0..configuration::width{
            for y in 0..configuration::height{
                for s in self.sampler.generate_samples(x, y) {
                    let ray = self.camera.generate_ray(x as f64, y as f64, s);
                    // let ray = self.camera.generate_ray(x as f64, y as f64);
                    let i = self.solver.solve(ray);
                    let col = self.shader.shade(x, y, i);
                    self.film.write_pixel(x, y, [col[0] * sample_inv, col[1] * sample_inv, col[2] * sample_inv]);
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