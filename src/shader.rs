use crate::configuration;
use crate::solver;
use std::{rc::Rc, cell::RefCell};
use crate::helpers;

pub trait Shader {
    fn surface_props(&self, x: u32, y: u32, i: &solver::IntersectionInfo) -> solver::SufraceProperties;
    fn miss_color(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3];
    fn is_lit(&self) -> bool;
    fn evaluate(&mut self, t: f64);
}
pub trait BackgroundShader {
    fn shade(&self, x: u32, y: u32) -> [f64;3];
    fn evaluate(&mut self, t: f64);
}

// ------------------------------------------
//                  COLOR
// ------------------------------------------

pub struct ColorShader{
    color: [f64;3],
    background_shader: Box<dyn BackgroundShader>
}
impl ColorShader{
    pub fn new(color: [f64;3], background_shader: Box<dyn BackgroundShader>) -> ColorShader{
        ColorShader{
            color,
            background_shader
        }
    }
}
impl Shader for ColorShader{
    fn surface_props(&self, x: u32, y: u32, i: &solver::IntersectionInfo) -> solver::SufraceProperties{
        return solver::SufraceProperties{color: self.color.clone()};
    }
    fn miss_color(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        return self.background_shader.shade(x, y);
    }
    fn evaluate(&mut self, t: f64){
        
    }
    fn is_lit(&self) -> bool{
        return false;
    }
}






// ------------------------------------------
//                  NORMAL
// ------------------------------------------

pub struct NormalShader{
    background_shader: Box<dyn BackgroundShader>
}
impl NormalShader{
    pub fn new(background_shader: Box<dyn BackgroundShader>) -> NormalShader{
        NormalShader{background_shader}
    }
}
impl Shader for NormalShader{
    fn surface_props(&self, x: u32, y: u32, i: &solver::IntersectionInfo) -> solver::SufraceProperties{
        return solver::SufraceProperties{color: [i.normal[0] * 0.5 + 0.5, i.normal[1] * 0.5 + 0.5, i.normal[2] * 0.5 + 0.5]}
    }
    fn miss_color(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        return self.background_shader.shade(x, y);
    }
    fn evaluate(&mut self, t: f64){
        
    }
    fn is_lit(&self) -> bool{
        return false;
    }
}








// ------------------------------------------
//                  FRACTAL
// ------------------------------------------

pub struct FractalShader{
    col1: [f64!();3],
    col2: [f64!();3],
    darkness: f64!(),
    light_dir: [f64!();3],
    background_shader: Box<dyn BackgroundShader>
}
impl FractalShader{
    pub fn new(col1: [f64!();3], col2: [f64!();3], darkness: f64!(), light_dir: [f64!();3], background_shader: Box<dyn BackgroundShader>) -> FractalShader{
        FractalShader{col1,col2,darkness,light_dir,background_shader}
    }
}
impl Shader for FractalShader{
    fn surface_props(&self, x: u32, y: u32, i: &solver::IntersectionInfo) -> solver::SufraceProperties{
        let col1 = get_f64v!(self.col1);
        let col2 = get_f64v!(self.col2);
        let light_dir = get_f64v!(self.light_dir);

        let a = vecmath::vec3_dot([i.normal[0] * 0.5 + 0.5, i.normal[1] * 0.5 + 0.5, i.normal[2] * 0.5 + 0.5], light_dir).clamp(0.0, 1.0);
        let b = (i.fractal_data[0] / 16.0).clamp(0.0, 1.0);
        let color_mix = helpers::vec_clamp([a * col1[0] + b * col2[0], a * col1[1] + b * col2[1], a * col1[2] + b * col2[2]], 0.0, 1.0);

        let rim = (i.steps as f64) / get_f64!(self.darkness);
        return solver::SufraceProperties{color: [color_mix[0] * rim, color_mix[1] * rim, color_mix[2] * rim]};
    }
    fn miss_color(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        return self.background_shader.shade(x, y);
    }
    fn evaluate(&mut self, t: f64){
        
    }
    fn is_lit(&self) -> bool{
        return false;
    }
}


// ------------------------------------------
// ------------------------------------------
// ------------------------------------------
// ------------------------------------------
//             LIGHT SHADER
// ------------------------------------------
// ------------------------------------------
// ------------------------------------------
// ------------------------------------------


// ------------------------------------------
//                  PHONG
// ------------------------------------------

pub struct PhongShader{
    background_shader: Box<dyn BackgroundShader>
}
impl PhongShader{
    pub fn new(background_shader: Box<dyn BackgroundShader>) -> PhongShader{
        PhongShader{background_shader}
    }
}
impl Shader for PhongShader{
    fn surface_props(&self, x: u32, y: u32, i: &solver::IntersectionInfo) -> solver::SufraceProperties{
        return solver::SufraceProperties{color: [1.0, 1.0, 1.0]};
    }
    fn miss_color(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        return self.background_shader.shade(x, y);
    }
    fn evaluate(&mut self, t: f64){
        
    }
    fn is_lit(&self) -> bool{
        return true;
    }
}

// ------------------------------------------
// ------------------------------------------
// ------------------------------------------
// ------------------------------------------
//               BAKCGROUND
// ------------------------------------------
// ------------------------------------------
// ------------------------------------------
// ------------------------------------------



pub struct BackgroundColorShader{
    color: [f64;3]
}
impl BackgroundColorShader{
    pub fn new(color: [f64;3]) -> BackgroundColorShader{
        BackgroundColorShader{
            color
        }
    }
}
impl BackgroundShader for BackgroundColorShader{
    fn shade(&self, x: u32, y: u32) -> [f64;3]{
        self.color.clone()
    }
    fn evaluate(&mut self, t: f64){
        
    }
}



pub struct BackgroundLinearXGradient{
    color1: [f64;3],
    color2: [f64;3]
}
impl BackgroundLinearXGradient{
    pub fn new(color1: [f64;3], color2: [f64;3]) -> BackgroundLinearXGradient{
        BackgroundLinearXGradient{
            color1,color2,
        }
    }
}
impl BackgroundShader for BackgroundLinearXGradient{
    fn shade(&self, x: u32, y: u32) -> [f64;3]{
        return helpers::vec_interpolate(self.color1, self.color2, x as f64 / configuration::width_f);
    }
    fn evaluate(&mut self, t: f64){
        
    }
}





pub struct BackgroundLinearYGradient{
    color1: [f64;3],
    color2: [f64;3],
}
impl BackgroundLinearYGradient{
    pub fn new(color1: [f64;3], color2: [f64;3]) -> BackgroundLinearYGradient{
        BackgroundLinearYGradient{
            color1,color2,
        }
    }
}
impl BackgroundShader for BackgroundLinearYGradient{
    fn shade(&self, x: u32, y: u32) -> [f64;3]{
        return helpers::vec_interpolate(self.color1, self.color2, y as f64 / configuration::height_f);
    }
    fn evaluate(&mut self, t: f64){
        
    }
}