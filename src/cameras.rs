use crate::CameraSample;
use crate::Sample;
use crate::configuration;
use crate::helpers;
use std::{rc::Rc, cell::RefCell};
pub trait Camera{
    fn generate_ray(&self, x: f64, y: f64, sample: CameraSample) -> crate::Ray;
    fn evaluate(&mut self, t: f64);
}

pub struct PinholeCamera{
    pos: [f64!(); 3],
    pos_f64: [f64;3],
    rot: [f64!(); 3],
    mat_inv: [[f64;3];3],
    ar: f64,
    scale: f64
}


impl Camera for PinholeCamera{
    fn generate_ray(&self, x: f64, y: f64, sample: CameraSample) -> crate::Ray{
        let rx = ((x / configuration::width_f - 0.5) * configuration::width_f) / configuration::height_f;
        let ry = (1.0 - y / configuration::height_f) - 0.5;
        
        //let rx = (2.0 * (x + 0.5) / configuration::width_f - 1.0) * self.ar * self.scale;
        //let ry = (1.0 - 2.0 * (y + 0.5) / configuration::height_f) * self.scale;
        let dir = vecmath::vec3_normalized([rx, ry, -1.0]);
        return (helpers::mat_dir_mul3(self.mat_inv, dir), [self.pos_f64[0], self.pos_f64[1],self.pos_f64[2]]);
    }
    fn evaluate(&mut self, t: f64){
        println!("{:?}", get_f64v!(self.rot));
        self.pos_f64 = [
            get_ref_f64!(self.pos[0]),
            get_ref_f64!(self.pos[1]),
            get_ref_f64!(self.pos[2]),
        ];
        self.mat_inv = vecmath::mat3_inv( helpers::mat_rotation3(get_f64v!(self.rot)));
    }
}

impl PinholeCamera{
    pub fn new(pos: [f64!(); 3], rot: [f64!(); 3]) -> PinholeCamera{
        let w_f = configuration::width_f as f64;
        let h_f = configuration::height_f as f64;
        let ar = w_f/h_f;
        let scale = (configuration::fov * 0.5).to_radians();
        let mat = vecmath::mat3_inv( helpers::mat_rotation3(get_f64v!(rot)));
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







pub struct ThinLensCamera{
    lens_radius: f64!(),
    focal_distance: f64!(),
    pos: [f64!(); 3],
    pos_f64: [f64;3],
    rot: [f64!(); 3],
    mat_inv: [[f64;4];4],
    ar: f64,
    scale: f64
}

impl Camera for ThinLensCamera{
    fn generate_ray(&self, x: f64, y: f64, sample: CameraSample) -> crate::Ray{
        let lens_radius = get_f64!(self.lens_radius);

        let rx = (2.0 * (x + 0.5) / configuration::width_f - 1.0) * self.ar * self.scale;
        let ry = (1.0 - 2.0 * (y + 0.5) / configuration::height_f) * self.scale;
        let dir = vecmath::vec3_normalized([rx, ry, -1.0]);
        // println!("{:?}", sample.1);
        let p_lens = helpers::concentric_sample_disk(sample.1.0, sample.1.1);
        let ft = get_f64!(self.focal_distance) / dir[2];

        let ray_o = [p_lens.0 * lens_radius, p_lens.1  * lens_radius, 0.0];
        
        let focus = [dir[0] * ft, dir[1] * ft, dir[2] * ft];
        // println!("{:?}", p_lens);
        return (helpers::mat_dir_mul(self.mat_inv,vecmath::vec3_normalized([ray_o[0] - focus[0], ray_o[1] - focus[1], -focus[2]])), [ray_o[0] + self.pos_f64[0], ray_o[0] + self.pos_f64[0], ray_o[1] + self.pos_f64[2]])
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

impl ThinLensCamera{
    pub fn new(lens_radius: f64!(), focal_distance: f64!(), pos: [f64!(); 3], rot: [f64!(); 3]) -> ThinLensCamera{
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
        ThinLensCamera{
            lens_radius,
            focal_distance,
            pos_f64,
            pos,
            rot, 
            ar,
            scale,
            mat_inv: mat
        }
    }
}