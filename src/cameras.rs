use crate::configuration;
use crate::helpers;
use std::{rc::Rc, cell::RefCell};
pub trait Camera{
    fn generate_ray(&self, x: f64, y: f64) -> crate::Ray;
    fn evaluate(&mut self, t: f64);
}

pub struct PinholeCamera{
    pos: [f64!(); 3],
    pos_f64: [f64;3],
    rot: [f64!(); 3],
    mat_inv: [[f64;4];4],
    ar: f64,
    scale: f64
}

impl Camera for PinholeCamera{
    fn generate_ray(&self, x: f64, y: f64) -> crate::Ray{
        let rx = (2.0 * (x + 0.5) / configuration::width_f - 1.0) * self.ar * self.scale;
        let ry = (1.0 - 2.0 * (y + 0.5) / configuration::height_f) * self.scale;
        let dir = vecmath::vec3_normalized([rx, ry, -1.0]);
        return (helpers::mat_dir_mul(self.mat_inv, dir), [self.pos_f64[0], self.pos_f64[1],self.pos_f64[2]])
    }
    fn evaluate(&mut self, t: f64){
        self.pos_f64 = [
            get_ref_f64!(self.pos[0]),
            get_ref_f64!(self.pos[1]),
            get_ref_f64!(self.pos[2]),
        ];
        self.mat_inv = vecmath::mat4_inv( helpers::mat_pos_rot(get_f64v!(self.pos), get_f64v!(self.rot)));
    }
}

impl PinholeCamera{
    pub fn new(pos: [f64!(); 3], rot: [f64!(); 3]) -> PinholeCamera{
        let w_f = configuration::width_f as f64;
        let h_f = configuration::height_f as f64;
        let ar = w_f/h_f;
        let scale = (configuration::fov * 0.5).to_radians();
        let mat = vecmath::mat4_inv( helpers::mat_pos_rot(get_f64v!(pos), get_f64v!(rot)));
        let pos_f64 = [
            get_f64!(pos[0]),
            get_f64!(pos[1]),
            get_f64!(pos[2])
        ];
        PinholeCamera{
            pos_f64,
            pos,
            rot, 
            ar,
            scale,
            mat_inv: mat
        }
    }
}