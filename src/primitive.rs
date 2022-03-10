use crate::helpers;
use crate::modifier;
use std::{rc::Rc, cell::RefCell};
use vecmath;

pub struct PrimitiveResult{
    pub distance: f64,
    pub fractal_data: [f64;3]
}

pub trait Primitive{
    fn map_primitive(&self, pos: [f64;3]) -> PrimitiveResult;
    fn evaluate(&mut self, t: f64);
}

pub trait InternalPrimitive{
    fn _map_primitive(&self, pos: [f64;3]) -> PrimitiveResult;
    fn get_primitive_data(&self) -> &PrimitiveData;
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData;
    fn _evaluate(&mut self, t: f64);
}

pub struct PrimitiveData{
    pos: [f64!();3],
    rot: [f64!();3],
    scale: [f64!();3],
    mat_inv: [[f64;4];4],
    pos_modifier: Vec<Box<dyn modifier::PosModifier>>
}

fn get_primitive_data(pos: [f64!();3], rot: [f64!();3], scale: [f64!();3], pos_modifier: Vec<Box<dyn modifier::PosModifier>>) -> PrimitiveData {
    let mat_inv = vecmath::mat4_inv(helpers::mat_transformation(get_ref_f64v!(pos), get_ref_f64v!(rot), get_ref_f64v!(scale)));
    PrimitiveData{
        pos,
        rot,
        scale,
        mat_inv,
        pos_modifier,
    }
}

impl<T> Primitive for T where T: InternalPrimitive{
    fn map_primitive(&self, pos: [f64;3]) -> PrimitiveResult{
        let data = self.get_primitive_data();
        let mut transformed_pos = helpers::mat_vec_mul(data.mat_inv, pos);

        for m in &data.pos_modifier{
            transformed_pos = m.modify(transformed_pos)
        }
        return self._map_primitive(transformed_pos);
    }

    fn evaluate(&mut self, t: f64){
        let data = self.get_primitive_data_mut();
        let mat_inv = vecmath::mat4_inv(helpers::mat_transformation(get_ref_f64v!(data.pos), get_ref_f64v!(data.rot), get_ref_f64v!(data.scale)));
        data.mat_inv = mat_inv;

        for pos_m in &data.pos_modifier{
            pos_m.evaluate(t);
        }

        self._evaluate(t)
    }
}

// ------------------------------------------
//                  SPHERE
// ------------------------------------------
pub struct Sphere{
    primtive_data: PrimitiveData,
    rad: f64!()
}

impl Sphere{
    pub fn new(rad: f64!(), pos: [f64!();3], rot: [f64!();3], scale: [f64!();3], pos_modifier: Vec<Box<dyn modifier::PosModifier>>) -> Sphere {
        Sphere{
            rad,
            primtive_data: get_primitive_data(pos, rot, scale, pos_modifier)
        }
    }
}

impl InternalPrimitive for Sphere{
    fn _map_primitive(&self, pos: [f64;3]) -> PrimitiveResult{
        let distance = vecmath::vec3_len(pos) - self.rad.take();
        PrimitiveResult{distance, fractal_data: [0.0, 0.0, 0.0]}
    }
    fn get_primitive_data(&self) -> &PrimitiveData{
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData{
        return &mut self.primtive_data;
    }

    fn _evaluate(&mut self, t: f64){
        
    }
}





// ------------------------------------------
//                   TORUS
// ------------------------------------------
pub struct Torus{
    primtive_data: PrimitiveData,
    rad: f64!(),
    ring_rad: f64!()
}

impl Torus{
    pub fn new(rad: f64!(), ring_rad: f64!(), pos: [f64!();3], rot: [f64!();3], scale: [f64!();3], pos_modifier: Vec<Box<dyn modifier::PosModifier>>) -> Torus {
        Torus{
            ring_rad,
            rad,
            primtive_data: get_primitive_data(pos, rot, scale, pos_modifier)
        }
    }
}

impl InternalPrimitive for Torus{
    fn _map_primitive(&self, pos: [f64;3]) -> PrimitiveResult{
        let l = (pos[0] * pos[0] + pos[2] * pos[2]).sqrt() - self.rad.take();
        let distance = (l * l + pos[1] * pos[1]).sqrt() - self.ring_rad.take();
        PrimitiveResult{distance, fractal_data: [0.0, 0.0, 0.0]}
    }
    fn get_primitive_data(&self) -> &PrimitiveData{
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData{
        return &mut self.primtive_data;
    }
    fn _evaluate(&mut self, t: f64){

    }
}






// ------------------------------------------
//                     Cube
// ------------------------------------------
pub struct Cube{
    primtive_data: PrimitiveData,
    bounds: [f64!();3]
}

impl Cube{
    pub fn new(bounds: [f64!();3], pos: [f64!();3], rot: [f64!();3], scale: [f64!();3], pos_modifier: Vec<Box<dyn modifier::PosModifier>>) -> Cube {
        Cube{
            bounds,
            primtive_data: get_primitive_data(pos, rot, scale, pos_modifier)
        }
    }
}

impl InternalPrimitive for Cube{
    fn _map_primitive(&self, pos: [f64;3]) -> PrimitiveResult{
        let dist_vec = [pos[0].abs() - self.bounds[0].take(), pos[1].abs() - self.bounds[1].take(), pos[2].abs() - self.bounds[2].take()];
        let distance = helpers::min_f64(helpers::max_f64(dist_vec[0],helpers::max_f64(dist_vec[1], dist_vec[2])), 0.0) + vecmath::vec3_len(helpers::vec_f_max(dist_vec, 0.0));
        PrimitiveResult{distance, fractal_data: [0.0, 0.0, 0.0]}
    }
    fn get_primitive_data(&self) -> &PrimitiveData{
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData{
        return &mut self.primtive_data;
    }
    fn _evaluate(&mut self, t: f64){

    }
}






// ------------------------------------------
//               Mandelbulb
// ------------------------------------------

pub struct Mandelbulb {
    primtive_data: PrimitiveData,
    power: f64!()
}

impl Mandelbulb{
    pub fn new(power: f64!(), pos: [f64!();3], rot: [f64!();3], scale: [f64!();3], pos_modifier: Vec<Box<dyn modifier::PosModifier>>) -> Mandelbulb {
        Mandelbulb{primtive_data: get_primitive_data(pos, rot, scale, pos_modifier), power: power}
    }
}

impl InternalPrimitive for Mandelbulb{
    fn _map_primitive(&self, pos: [f64;3]) -> PrimitiveResult{
        let mut z = [pos[0], pos[1], pos[2]];
        let mut dr = 1.0;
        let mut r = 0.0;
        let mut iterations = 0;
        // println!("{}",self.power.take());
        let pow = *self.power.as_ref().borrow();

        for i in 0..15{
            iterations = i;
            r = vecmath::vec3_len(z);

            if r > 2.0 {
                break
            }
            
            let mut theta = (z[2]/r).acos();
            let mut phi = z[1].atan2(z[0]);
            dr = r.powf(pow - 1.0) * pow * dr + 1.0;

            let zr = r.powf(pow);
            theta = theta*pow;
            phi = phi*pow;
            

            z = [theta.sin() * phi.cos() * zr, phi.sin() * theta.sin() * zr, theta.cos() * zr];
            z = [z[0] + pos[0], z[1] + pos[1], z[2] + pos[2]];
        }
        let distance = 0.5 * r.log(std::f64::consts::E) * r / dr;
        PrimitiveResult{distance, fractal_data: [iterations as f64, iterations as f64, iterations as f64]}
    }
    fn get_primitive_data(&self) -> &PrimitiveData{
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData{
        return &mut self.primtive_data;
    }
    fn _evaluate(&mut self, t: f64){
        // println!("{}", self.power.take());
    }
}