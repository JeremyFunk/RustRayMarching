use crate::cameras;
use crate::film;
use crate::solver;
use crate::shader;
use crate::configuration::Config;

pub trait Renderer{
    fn render(&mut self);
    fn prepare_render(&mut self);
    fn get_image(&self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;
    fn evaluate(&mut self, t: f64);
}

pub struct CameraRayRenderer<C: cameras::Camera, F: film::Film> {
    camera: C,
    film: F
}

impl<C: cameras::Camera, F: film::Film> Renderer for CameraRayRenderer<C, F>{
    fn render(&mut self){
        for x in 0..Config.width{
            for y in 0..Config.height{
                let ray = self.camera.generate_ray(x as f64, y as f64);
                self.film.write_pixel(x, y, ray.0);
            }
        }
    }
    fn prepare_render(&mut self){

    }
    fn get_image(&self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>{
        return self.film.build_image();
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










pub struct SolverRenderer<C: cameras::Camera, F: film::Film, S: solver::Solver, H: shader::Shader> {
    camera: C,
    film: F,
    solver: S,
    shader: H
}

impl<C: cameras::Camera, F: film::Film, S: solver::Solver, H: shader::Shader> Renderer for SolverRenderer<C, F, S, H>{
    fn render(&mut self){
        for x in 0..Config.width{
            for y in 0..Config.height{
                let ray = self.camera.generate_ray(x as f64, y as f64);
                let i = self.solver.solve(ray);
                let col = self.shader.shade(x, y, i);
                self.film.write_pixel(x, y, col);
            }
        }
    }
    fn prepare_render(&mut self){
        self. film.prepare_render()
    }
    fn get_image(&self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>{
        return self.film.build_image();
    }
    fn evaluate(&mut self, t: f64){
        self.camera.evaluate(t);
        self.film.evaluate(t);
        self.solver.evaluate(t);
        self.shader.evaluate(t);
    }
}

impl<C: cameras::Camera, F: film::Film, S: solver::Solver, H: shader::Shader> SolverRenderer<C, F, S, H>{
    pub fn new(camera: C, film: F, solver: S, shader: H) -> SolverRenderer<C, F, S, H>{
        SolverRenderer{
            camera: camera,
            film: film,
            solver,
            shader
        }
    }
}