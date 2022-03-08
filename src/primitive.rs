use crate::helpers;
use crate::modifier;
use vecmath;
pub trait Primitive{
    fn map_primitive(&self, pos: [f64;3]) -> f64;
}

pub trait InternalPrimitive{
    fn _map_primitive(&self, pos: [f64;3]) -> f64;
    fn get_primitive_data(&self) -> &PrimitiveData;
}

pub struct PrimitiveData{
    pos: [f64;3],
    rot: [f64;3],
    scale: [f64;3],
    mat_inv: [[f64;4];4],
    pos_modifier: Vec<Box<dyn modifier::PosModifier>>
}

impl<T> Primitive for T where T: InternalPrimitive{
    fn map_primitive(&self, pos: [f64;3]) -> f64{
        let data = self.get_primitive_data();
        let mut transformed_pos = helpers::mat_vec_mul(data.mat_inv, pos);

        for m in &data.pos_modifier{
            transformed_pos = m.modify(transformed_pos)
        }
        return self._map_primitive(transformed_pos);
    }
}

// ------------------------------------------
//                  SPHERE
// ------------------------------------------
pub struct Sphere{
    primtive_data: PrimitiveData,
    rad: f64
}

impl Sphere{
    pub fn new(rad: f64, pos: [f64;3], rot: [f64;3], scale: [f64;3], pos_modifier: Vec<Box<dyn modifier::PosModifier>>) -> Sphere {
        let mat_inv = vecmath::mat4_inv(helpers::mat_transformation(pos, rot, scale));
        Sphere{
            rad,
            primtive_data: PrimitiveData{
                pos,rot,scale,mat_inv,pos_modifier
            }
        }
    }
}

impl InternalPrimitive for Sphere{
    fn _map_primitive(&self, pos: [f64;3]) -> f64{
        let distance = vecmath::vec3_len(pos) - self.rad;
        distance
    }
    fn get_primitive_data(&self) -> &PrimitiveData{
        return &self.primtive_data;
    }
}





// ------------------------------------------
//                   TORUS
// ------------------------------------------
pub struct Torus{
    primtive_data: PrimitiveData,
    rad: f64,
    ring_rad: f64
}

impl Torus{
    pub fn new(rad: f64, ring_rad: f64, pos: [f64;3], rot: [f64;3], scale: [f64;3], pos_modifier: Vec<Box<dyn modifier::PosModifier>>) -> Torus {
        let mat_inv = vecmath::mat4_inv(helpers::mat_transformation(pos, rot, scale));
        Torus{
            ring_rad,
            rad,
            primtive_data: PrimitiveData{
                pos,rot,scale,mat_inv,pos_modifier
            }
        }
    }
}

impl InternalPrimitive for Torus{
    fn _map_primitive(&self, pos: [f64;3]) -> f64{
        let l = (pos[0] * pos[0] + pos[2] * pos[2]).sqrt() - self.rad;
        return (l * l + pos[1] * pos[1]).sqrt() - self.ring_rad
    }
    fn get_primitive_data(&self) -> &PrimitiveData{
        return &self.primtive_data;
    }
}






// ------------------------------------------
//                     Cube
// ------------------------------------------
pub struct Cube{
    primtive_data: PrimitiveData,
    bounds: [f64;3]
}

impl Cube{
    pub fn new(bounds: [f64;3], pos: [f64;3], rot: [f64;3], scale: [f64;3], pos_modifier: Vec<Box<dyn modifier::PosModifier>>) -> Cube {
        let mat_inv = vecmath::mat4_inv(helpers::mat_transformation(pos, rot, scale));
        Cube{
            bounds,
            primtive_data: PrimitiveData{
                pos,rot,scale,mat_inv,pos_modifier
            }
        }
    }
}

impl InternalPrimitive for Cube{
    fn _map_primitive(&self, pos: [f64;3]) -> f64{
        let dist_vec = [pos[0].abs() - self.bounds[0], pos[1].abs() - self.bounds[1], pos[2].abs() - self.bounds[2]];
        return helpers::min_f64(helpers::max_f64(dist_vec[0],helpers::max_f64(dist_vec[1], dist_vec[2])), 0.0) + vecmath::vec3_len(helpers::vec_f_max(dist_vec, 0.0));
    }
    fn get_primitive_data(&self) -> &PrimitiveData{
        return &self.primtive_data;
    }
}