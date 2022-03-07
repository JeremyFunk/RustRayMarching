use crate::configuration::Config;
use crate::solver;

pub trait Shader {
    fn shade(&self, x: u32, y: u32, i: solver::IntersectionInfo) -> [f64;3];
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
}




