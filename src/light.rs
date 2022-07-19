
use std::{rc::Rc, cell::RefCell, f64::consts::PI};
use vecmath;
pub struct LightInfo{
    pub distance: f64,
    pub direction: [f64;3],
    pub light_intensity: [f64;3]
}

pub trait Light{
    fn illuminate(&self, point: [f64;3]) -> LightInfo;
}


pub struct DirectionalLight {
    direction: [f64!();3],
    color: [f64!();3],
    intensity: f64!(),
}

impl Light for DirectionalLight{
    fn illuminate(&self, point: [f64;3]) -> LightInfo {
        LightInfo { distance: f64::MAX, direction: vecmath::vec3_normalized([-get_f64!(self.direction[0]), -get_f64!(self.direction[1]),-get_f64!(self.direction[2])]), light_intensity: [get_f64!(self.color[0]) * get_f64!(self.intensity), get_f64!(self.color[1]) * get_f64!(self.intensity), get_f64!(self.color[2]) * get_f64!(self.intensity)]}
    }
}

impl DirectionalLight{
    pub fn new(direction: [f64!();3], color: [f64!();3], intensity: f64!()) -> DirectionalLight{
        DirectionalLight { direction: direction, color, intensity }
    }
}



pub struct PointLight {
    position: [f64!();3],
    color: [f64!();3],
    intensity: f64!(),
}

impl Light for PointLight{
    fn illuminate(&self, point: [f64;3]) -> LightInfo {
        let mut light_dir = [get_f64!(self.position[0]) - point[0], get_f64!(self.position[1]) - point[1], get_f64!(self.position[2]) - point[2]];
        let r2 = vecmath::vec3_square_len(light_dir);
        let distance = r2.sqrt();
        light_dir = [light_dir[0] / distance, light_dir[1] / distance, light_dir[2] / distance];
        let mul = 1.0 / (4.0 * PI * r2);
        let intensity = [get_f64!(self.color[0]) * get_f64!(self.intensity) * mul, get_f64!(self.color[1]) * get_f64!(self.intensity) * mul, get_f64!(self.color[2]) * get_f64!(self.intensity) * mul];

        LightInfo { distance: distance, direction: light_dir, light_intensity: intensity }
    }
}

impl PointLight{
    pub fn new(position: [f64!();3], color: [f64!();3], intensity: f64!()) -> PointLight{
        PointLight { position, color, intensity }
    }
}