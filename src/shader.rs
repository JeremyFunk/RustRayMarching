use crate::configuration::Config;
use crate::solver;
use crate::helpers;

pub trait Shader {
    fn shade(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3];
    fn evaluate(&mut self, t: f64);
}


// ------------------------------------------
//                  COLOR
// ------------------------------------------

pub struct ColorShader{
    color: [f64;3]
}
impl ColorShader{
    pub fn new(color: [f64;3]) -> ColorShader{
        ColorShader{
            color
        }
    }
}
impl Shader for ColorShader{
    fn shade(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        if(i.hit){
            return self.color.clone()
        }
        return [0.0, 0.0, 0.0];
    }
    fn evaluate(&mut self, t: f64){
        
    }
}






// ------------------------------------------
//                  NORMAL
// ------------------------------------------

pub struct NormalShader{}
impl NormalShader{
    pub fn new() -> NormalShader{
        NormalShader{}
    }
}
impl Shader for NormalShader{
    fn shade(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        if(i.hit){
            return [i.normal[0] * 0.5 + 0.5, i.normal[1] * 0.5 + 0.5, i.normal[2] * 0.5 + 0.5]
        }
        return [0.0, 0.0, 0.0];
    }
    fn evaluate(&mut self, t: f64){
        
    }
}






// ------------------------------------------
//                  FRACTAL
// ------------------------------------------

pub struct FractalShader{
    col1: [f64;3],
    col2: [f64;3],
    darkness: f64,
    light_dir: [f64;3],
}
impl FractalShader{
    pub fn new(col1: [f64;3], col2: [f64;3], darkness: f64, light_dir: [f64;3]) -> FractalShader{
        FractalShader{col1,col2,darkness,light_dir}
    }
}
impl Shader for FractalShader{
    fn shade(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3]{
        if(i.hit){
            let a = vecmath::vec3_dot([i.normal[0] * 0.5 + 0.5, i.normal[1] * 0.5 + 0.5, i.normal[2] * 0.5 + 0.5], self.light_dir).clamp(0.0, 1.0);
            let b = (i.fractal_data[0] / 16.0).clamp(0.0, 1.0);
            let color_mix = helpers::vec_clamp([a * self.col1[0] + b * self.col2[0], a * self.col1[1] + b * self.col2[1], a * self.col1[2] + b * self.col2[2]], 0.0, 1.0);

            let rim = (i.steps as f64) / self.darkness;
            return [color_mix[0] * rim, color_mix[1] * rim, color_mix[2] * rim]
        }
        return [0.0, 0.0, 0.0];
    }
    fn evaluate(&mut self, t: f64){
        
    }
}




