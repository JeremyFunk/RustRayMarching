use crate::configuration::Config;
use crate::solver;
use std::{rc::Rc, cell::RefCell};
use crate::helpers;

pub trait Shader {
    fn shade(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3];
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
    fn shade(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        if(i.hit){
            return self.color.clone()
        }
        return self.background_shader.shade(x, y);
    }
    fn evaluate(&mut self, t: f64){
        
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
    fn shade(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        if(i.hit){
            return [i.normal[0] * 0.5 + 0.5, i.normal[1] * 0.5 + 0.5, i.normal[2] * 0.5 + 0.5]
        }
        return self.background_shader.shade(x, y);
    }
    fn evaluate(&mut self, t: f64){
        
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
    fn shade(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        let col1 = get_f64v!(self.col1);
        let col2 = get_f64v!(self.col2);
        let light_dir = get_f64v!(self.light_dir);

        if(i.hit){
            let a = vecmath::vec3_dot([i.normal[0] * 0.5 + 0.5, i.normal[1] * 0.5 + 0.5, i.normal[2] * 0.5 + 0.5], light_dir).clamp(0.0, 1.0);
            let b = (i.fractal_data[0] / 16.0).clamp(0.0, 1.0);
            let color_mix = helpers::vec_clamp([a * col1[0] + b * col2[0], a * col1[1] + b * col2[1], a * col1[2] + b * col2[2]], 0.0, 1.0);

            let rim = (i.steps as f64) / get_f64!(self.darkness);
            return [color_mix[0] * rim, color_mix[1] * rim, color_mix[2] * rim]
        }
        return self.background_shader.shade(x, y);
    }
    fn evaluate(&mut self, t: f64){
        
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
        return helpers::vec_interpolate(self.color1, self.color2, x as f64 / Config.width_f);
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
        return helpers::vec_interpolate(self.color1, self.color2, y as f64 / Config.height_f);
    }
    fn evaluate(&mut self, t: f64){
        
    }
}