use crate::configuration;
use crate::helpers;
use crate::helpers::min_f64;
use crate::modifier;
use std::{cell::RefCell, rc::Rc};
use vecmath;

pub enum GroupModifierType {
    Union = 1,
    Subtraction = 2,
    Intersection = 3,
    SmoothUnion = 4,
    SmoothSubtraction = 5,
    SmoothIntersection = 6,
}
pub struct Material {
    pub albedo: f64!(),
    pub specular: f64!(),
    pub diffuse: f64!(),
    pub n_specular: f64!(),
}
#[derive(Clone, Copy)]
pub struct MaterialRaw {
    pub albedo: f64,
    pub specular: f64,
    pub diffuse: f64,
    pub n_specular: f64,
}

impl MaterialRaw {
    pub fn new(
        albedo: f64!(),
        specular: f64!(),
        diffuse: f64!(),
        n_specular: f64!(),
    ) -> MaterialRaw {
        MaterialRaw {
            albedo: get_ref_f64!(albedo),
            diffuse: get_ref_f64!(diffuse),
            n_specular: get_ref_f64!(n_specular),
            specular: get_ref_f64!(specular),
        }
    }
}

impl Material {
    pub fn new(albedo: f64!(), specular: f64!(), diffuse: f64!(), n_specular: f64!()) -> Material {
        Material {
            albedo,
            specular,
            diffuse,
            n_specular,
        }
    }

    pub fn new_base() -> Material {
        Material {
            albedo: f64!(0.18),
            specular: f64!(0.2),
            diffuse: f64!(0.8),
            n_specular: f64!(10.0),
        }
    }

    pub fn new_direct() -> Material {
        Material {
            albedo: f64!(1.0),
            specular: f64!(0.0),
            diffuse: f64!(1.0),
            n_specular: f64!(0.0),
        }
    }
}

pub struct PrimitiveResult {
    pub distance: f64,
    pub fractal_data: [f64; 4],
    pub material: MaterialRaw,
}

pub trait Primitive {
    /* Used to avoid mapping position in wrong direction. Example:

        Some point:
                        .
        Gets mapped by modifiers:
                      .

        And trace:
        |
        |
        |<-<-<-<-<-<-<
        |
        |

        => This point gets moved by the normal

        |
        |
        | -> .
        |
        |

        => But the modifier moves the value back behind the object in that scenario:

        |
        |
       .|
        |
        |
    */
    fn map_primitive_mapped(&self, pos: [f64; 3]) -> PrimitiveResult;
    fn map_primitive(&self, pos: [f64; 3]) -> PrimitiveResult;
    fn evaluate(&mut self, t: f64);
}

pub trait InternalPrimitive {
    fn _map_primitive(&self, pos: [f64; 3]) -> PrimitiveResult;
    fn get_primitive_data(&self) -> &PrimitiveData;
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData;
    fn _evaluate(&mut self, t: f64);
    fn _use_raw_position(&self) -> bool;
}

pub struct PrimitiveData {
    pos: [f64!(); 3],
    rot: [f64!(); 3],
    scale: [f64!(); 3],
    mat_inv: [[f64; 4]; 4],
    pos_modifier: Vec<Box<dyn modifier::PosModifier>>,
}

fn get_primitive_data(
    pos: [f64!(); 3],
    rot: [f64!(); 3],
    scale: [f64!(); 3],
    pos_modifier: Vec<Box<dyn modifier::PosModifier>>,
) -> PrimitiveData {
    let mat_inv = vecmath::mat4_inv(helpers::mat_transformation(
        get_ref_f64v!(pos),
        get_ref_f64v!(rot),
        get_ref_f64v!(scale),
    ));
    PrimitiveData {
        pos,
        rot,
        scale,
        mat_inv,
        pos_modifier,
    }
}

impl<T> Primitive for T
where
    T: InternalPrimitive,
{
    fn map_primitive(&self, pos: [f64; 3]) -> PrimitiveResult {
        if !self._use_raw_position() {
            let data = self.get_primitive_data();
            let mut transformed_pos = helpers::mat_vec_mul(data.mat_inv, pos);

            for m in &data.pos_modifier {
                transformed_pos = m.modify(transformed_pos)
            }
            return self._map_primitive(transformed_pos);
        }
        return self._map_primitive(pos);
    }
    
    fn map_primitive_mapped(&self, pos: [f64; 3]) -> PrimitiveResult {
        return self._map_primitive(pos);
    }

    fn evaluate(&mut self, t: f64) {
        let data = self.get_primitive_data_mut();
        let mat_inv = vecmath::mat4_inv(helpers::mat_transformation(
            get_ref_f64v!(data.pos),
            get_ref_f64v!(data.rot),
            get_ref_f64v!(data.scale),
        ));
        data.mat_inv = mat_inv;

        for pos_m in &data.pos_modifier {
            pos_m.evaluate(t);
        }

        self._evaluate(t)
    }
}

// ------------------------------------------
//                  SPHERE
// ------------------------------------------

pub struct GroupPrimitive {
    primtive_data: PrimitiveData,
    group_primitive_type: GroupModifierType,
    prims: Vec<Box<dyn Primitive>>,
    factor_f: f64!(),
    factor: f64,
}

impl GroupPrimitive {
    pub fn new_union(prims: Vec<Box<dyn Primitive>>) -> GroupPrimitive {
        GroupPrimitive {
            group_primitive_type: GroupModifierType::Union,
            prims: prims,
            primtive_data: get_primitive_data(
                f64v!([0.0, 0.0, 0.0]),
                f64v!([0.0, 0.0, 0.0]),
                f64v!([1.0, 1.0, 1.0]),
                Vec::new(),
            ),
            factor_f: f64!(0.0),
            factor: 0.0,
        }
    }
    pub fn new_subtraction(prims: Vec<Box<dyn Primitive>>) -> GroupPrimitive {
        GroupPrimitive {
            group_primitive_type: GroupModifierType::Subtraction,
            prims: prims,
            primtive_data: get_primitive_data(
                f64v!([0.0, 0.0, 0.0]),
                f64v!([0.0, 0.0, 0.0]),
                f64v!([1.0, 1.0, 1.0]),
                Vec::new(),
            ),
            factor_f: f64!(0.0),
            factor: 0.0,
        }
    }
    pub fn new_intersection(prims: Vec<Box<dyn Primitive>>) -> GroupPrimitive {
        GroupPrimitive {
            group_primitive_type: GroupModifierType::Intersection,
            prims: prims,
            primtive_data: get_primitive_data(
                f64v!([0.0, 0.0, 0.0]),
                f64v!([0.0, 0.0, 0.0]),
                f64v!([1.0, 1.0, 1.0]),
                Vec::new(),
            ),
            factor_f: f64!(0.0),
            factor: 0.0,
        }
    }
    pub fn new_union_smooth(prims: Vec<Box<dyn Primitive>>, factor: f64!()) -> GroupPrimitive {
        let f = get_f64!(factor);
        GroupPrimitive {
            group_primitive_type: GroupModifierType::SmoothUnion,
            prims: prims,
            primtive_data: get_primitive_data(
                f64v!([0.0, 0.0, 0.0]),
                f64v!([0.0, 0.0, 0.0]),
                f64v!([1.0, 1.0, 1.0]),
                Vec::new(),
            ),
            factor: f,
            factor_f: factor,
        }
    }
    pub fn new_subtraction_smooth(
        prims: Vec<Box<dyn Primitive>>,
        factor: f64!(),
    ) -> GroupPrimitive {
        let f = get_f64!(factor);
        GroupPrimitive {
            group_primitive_type: GroupModifierType::SmoothSubtraction,
            prims: prims,
            primtive_data: get_primitive_data(
                f64v!([0.0, 0.0, 0.0]),
                f64v!([0.0, 0.0, 0.0]),
                f64v!([1.0, 1.0, 1.0]),
                Vec::new(),
            ),
            factor: f,
            factor_f: factor,
        }
    }
    pub fn new_intersection_smooth(
        prims: Vec<Box<dyn Primitive>>,
        factor: f64!(),
    ) -> GroupPrimitive {
        let f = get_f64!(factor);
        GroupPrimitive {
            group_primitive_type: GroupModifierType::SmoothIntersection,
            prims: prims,
            primtive_data: get_primitive_data(
                f64v!([0.0, 0.0, 0.0]),
                f64v!([0.0, 0.0, 0.0]),
                f64v!([1.0, 1.0, 1.0]),
                Vec::new(),
            ),
            factor: f,
            factor_f: factor,
        }
    }
}

impl InternalPrimitive for GroupPrimitive {
    fn _map_primitive(&self, pos: [f64; 3]) -> PrimitiveResult {
        let mut distance = self.prims[0].map_primitive(pos);
        // if(!distance.distance.is_nan()){

        // }
        match self.group_primitive_type {
            GroupModifierType::Intersection => {
                for p in 1..self.prims.len() {
                    let cur_dist = self.prims[p].map_primitive(pos);
                    if cur_dist.distance > distance.distance {
                        distance = cur_dist;
                    }
                }
            }
            GroupModifierType::Union => {
                for p in 1..self.prims.len() {
                    let cur_dist = self.prims[p].map_primitive(pos);
                    if cur_dist.distance < distance.distance {
                        distance = cur_dist;
                    }
                }
            }
            GroupModifierType::Subtraction => {
                distance.distance = -distance.distance;
                for p in 1..self.prims.len() {
                    let cur_dist = self.prims[p].map_primitive(pos);
                    if cur_dist.distance > distance.distance {
                        distance = cur_dist;
                    }
                }
            }
            GroupModifierType::SmoothUnion => {
                for p in 1..self.prims.len() {
                    let cur_dist = self.prims[p].map_primitive(pos);
                    let mut h = 0.5 + 0.5 * (cur_dist.distance - distance.distance) / self.factor;
                    if h < 0.0 {
                        h = 0.0;
                    }
                    if h > 1.0 {
                        h = 1.0;
                    }
                    distance.distance = helpers::mix(cur_dist.distance, distance.distance, h)
                        - self.factor * h * (1.0 - h);
                }
            }
            GroupModifierType::SmoothSubtraction => {
                for p in 1..self.prims.len() {
                    let cur_dist = self.prims[p].map_primitive(pos);
                    let mut h = 0.5 - 0.5 * (cur_dist.distance + distance.distance) / self.factor;
                    if h < 0.0 {
                        h = 0.0;
                    }
                    if h > 1.0 {
                        h = 1.0;
                    }
                    distance.distance = helpers::mix(cur_dist.distance, -distance.distance, h)
                        + self.factor * h * (1.0 - h);
                }
            }
            GroupModifierType::SmoothIntersection => {
                for p in 1..self.prims.len() {
                    let cur_dist = self.prims[p].map_primitive(pos);
                    let mut h = 0.5 - 0.5 * (cur_dist.distance - distance.distance) / self.factor;
                    if h < 0.0 {
                        h = 0.0;
                    }
                    if h > 1.0 {
                        h = 1.0;
                    }
                    distance.distance = helpers::mix(cur_dist.distance, distance.distance, h)
                        + self.factor * h * (1.0 - h);
                }
            }
        }
        distance
    }
    fn get_primitive_data(&self) -> &PrimitiveData {
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData {
        return &mut self.primtive_data;
    }

    fn _evaluate(&mut self, t: f64) {
        for p in &mut self.prims {
            p.evaluate(t);
        }
        self.factor = get_ref_f64!(self.factor_f);
    }
    fn _use_raw_position(&self) -> bool {
        return true;
    }
}

// ------------------------------------------
//                  SPHERE
// ------------------------------------------
pub struct Sphere {
    primtive_data: PrimitiveData,
    rad: f64!(),
    material: Material,
    material_raw: MaterialRaw,
}

impl Sphere {
    pub fn new(
        rad: f64!(),
        pos: [f64!(); 3],
        rot: [f64!(); 3],
        scale: [f64!(); 3],
        pos_modifier: Vec<Box<dyn modifier::PosModifier>>,
        material: Option<Material>,
    ) -> Sphere {
        let m = material.unwrap_or(Material::new_direct());
        Sphere {
            rad,
            primtive_data: get_primitive_data(pos, rot, scale, pos_modifier),
            material_raw: MaterialRaw::new(
                m.albedo.clone(),
                m.specular.clone(),
                m.diffuse.clone(),
                m.n_specular.clone(),
            ),
            material: m,
        }
    }
}

impl InternalPrimitive for Sphere {
    fn _map_primitive(&self, pos: [f64; 3]) -> PrimitiveResult {
        let distance = vecmath::vec3_len(pos) - get_f64!(self.rad);
        PrimitiveResult {
            distance,
            fractal_data: [0.0, 0.0, 0.0, 0.0],
            material: self.material_raw
        }
    }
    fn get_primitive_data(&self) -> &PrimitiveData {
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData {
        return &mut self.primtive_data;
    }

    fn _evaluate(&mut self, t: f64) {}
    fn _use_raw_position(&self) -> bool {
        return false;
    }
}

// ------------------------------------------
//                   TORUS
// ------------------------------------------
pub struct Torus {
    primtive_data: PrimitiveData,
    rad: f64!(),
    ring_rad: f64!(),
    material: Material,
    material_raw: MaterialRaw,
}

impl Torus {
    pub fn new(
        rad: f64!(),
        ring_rad: f64!(),
        pos: [f64!(); 3],
        rot: [f64!(); 3],
        scale: [f64!(); 3],
        pos_modifier: Vec<Box<dyn modifier::PosModifier>>,
        material: Option<Material>,
    ) -> Torus {
        let m = material.unwrap_or(Material::new_direct());
        Torus {
            ring_rad,
            rad,
            primtive_data: get_primitive_data(pos, rot, scale, pos_modifier),
            material_raw: MaterialRaw::new(
                m.albedo.clone(),
                m.specular.clone(),
                m.diffuse.clone(),
                m.n_specular.clone(),
            ),
            material: m,
        }
    }
}

impl InternalPrimitive for Torus {
    fn _map_primitive(&self, pos: [f64; 3]) -> PrimitiveResult {
        let l = (pos[0] * pos[0] + pos[2] * pos[2]).sqrt() - get_f64!(self.rad);
        let distance = (l * l + pos[1] * pos[1]).sqrt() - get_f64!(self.ring_rad);
        PrimitiveResult {
            distance,
            fractal_data: [0.0, 0.0, 0.0, 0.0],
            material: self.material_raw
        }
    }
    fn get_primitive_data(&self) -> &PrimitiveData {
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData {
        return &mut self.primtive_data;
    }
    fn _evaluate(&mut self, t: f64) {}
    fn _use_raw_position(&self) -> bool {
        return false;
    }
}

// ------------------------------------------
//                     Cube
// ------------------------------------------
pub struct Cube {
    primtive_data: PrimitiveData,
    bounds: [f64!(); 3],
    material: Material,
    material_raw: MaterialRaw,
}

impl Cube {
    pub fn new(
        bounds: [f64!(); 3],
        pos: [f64!(); 3],
        rot: [f64!(); 3],
        scale: [f64!(); 3],
        pos_modifier: Vec<Box<dyn modifier::PosModifier>>,
        material: Option<Material>,
    ) -> Cube {
        let m = material.unwrap_or(Material::new_direct());
        Cube {
            bounds,
            primtive_data: get_primitive_data(pos, rot, scale, pos_modifier),
            material_raw: MaterialRaw::new(
                m.albedo.clone(),
                m.specular.clone(),
                m.diffuse.clone(),
                m.n_specular.clone(),
            ),
            material: m,
        }
    }
}

impl InternalPrimitive for Cube {
    fn _map_primitive(&self, pos: [f64; 3]) -> PrimitiveResult {
        let dist_vec = [
            pos[0].abs() - get_f64!(self.bounds[0]),
            pos[1].abs() - get_f64!(self.bounds[1]),
            pos[2].abs() - get_f64!(self.bounds[2]),
        ];
        let distance = helpers::min_f64(
            helpers::max_f64(dist_vec[0], helpers::max_f64(dist_vec[1], dist_vec[2])),
            0.0,
        ) + vecmath::vec3_len(helpers::vec_f_max(dist_vec, 0.0));
        PrimitiveResult {
            distance,
            fractal_data: [0.0, 0.0, 0.0, 0.0],
            material: self.material_raw
        }
    }
    fn get_primitive_data(&self) -> &PrimitiveData {
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData {
        return &mut self.primtive_data;
    }
    fn _evaluate(&mut self, t: f64) {}
    fn _use_raw_position(&self) -> bool {
        return false;
    }
}

// ------------------------------------------
//               Mandelbulb
// ------------------------------------------

pub struct Mandelbulb {
    primtive_data: PrimitiveData,
    power: f64!(),
    material: Material,
    material_raw: MaterialRaw,
}

impl Mandelbulb {
    pub fn new(
        power: f64!(),
        pos: [f64!(); 3],
        rot: [f64!(); 3],
        scale: [f64!(); 3],
        pos_modifier: Vec<Box<dyn modifier::PosModifier>>,
    ) -> Mandelbulb {
        let m = Material::new_direct();
        Mandelbulb {
            primtive_data: get_primitive_data(pos, rot, scale, pos_modifier),
            power: power,
            material_raw: MaterialRaw::new(
                m.albedo.clone(),
                m.specular.clone(),
                m.diffuse.clone(),
                m.n_specular.clone(),
            ),
            material: m,
        }
    }
}

impl InternalPrimitive for Mandelbulb {
    fn _map_primitive(&self, pos: [f64; 3]) -> PrimitiveResult {
        let mut z = [pos[0], pos[1], pos[2]];
        let mut dr = 1.0;
        let mut r = 0.0;
        let mut iterations = 0;
        let pow = *self.power.as_ref().borrow();

        for i in 0..15 {
            iterations = i;
            r = vecmath::vec3_len(z);

            if r > 2.0 {
                break;
            }

            let mut theta = (z[2] / r).acos();
            let mut phi = z[1].atan2(z[0]);
            dr = r.powf(pow - 1.0) * pow * dr + 1.0;

            let zr = r.powf(pow);
            theta = theta * pow;
            phi = phi * pow;

            z = [
                theta.sin() * phi.cos() * zr,
                phi.sin() * theta.sin() * zr,
                theta.cos() * zr,
            ];
            z = [z[0] + pos[0], z[1] + pos[1], z[2] + pos[2]];
        }
        let distance = 0.5 * r.log(std::f64::consts::E) * r / dr;
        PrimitiveResult {
            distance,
            fractal_data: [
                iterations as f64,
                iterations as f64,
                iterations as f64,
                iterations as f64,
            ],
            material: self.material_raw
        }
    }
    fn get_primitive_data(&self) -> &PrimitiveData {
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData {
        return &mut self.primtive_data;
    }
    fn _evaluate(&mut self, t: f64) {}
    fn _use_raw_position(&self) -> bool {
        return false;
    }
}

// ------------------------------------------
//                  Julia
// ------------------------------------------

pub struct Julia {
    primtive_data: PrimitiveData,
    power: [f64!(); 4],
    material: Material,
    material_raw: MaterialRaw,
}

impl Julia {
    pub fn new(
        power: [f64!(); 4],
        pos: [f64!(); 3],
        rot: [f64!(); 3],
        scale: [f64!(); 3],
        pos_modifier: Vec<Box<dyn modifier::PosModifier>>,
    ) -> Julia {
        let m = Material::new_direct();
        Julia {
            primtive_data: get_primitive_data(pos, rot, scale, pos_modifier),
            power: power,
            material_raw: MaterialRaw::new(
                m.albedo.clone(),
                m.specular.clone(),
                m.diffuse.clone(),
                m.n_specular.clone(),
            ),
            material: m,
        }
    }
}

impl InternalPrimitive for Julia {
    fn _map_primitive(&self, pos: [f64; 3]) -> PrimitiveResult {
        let mut z = [pos[0], pos[1], pos[2], 0.0];
        let mut md2 = 1.0;
        let mut mz2 = vecmath::vec4_dot(z, z);

        let mut trap = [z[0].abs(), z[1].abs(), z[2].abs(), mz2];

        let mut n = 1.0;

        let power = [
            *self.power[0].as_ref().borrow(),
            *self.power[1].as_ref().borrow(),
            *self.power[2].as_ref().borrow(),
            *self.power[3].as_ref().borrow(),
        ];

        for i in 0..11 {
            md2 *= 4.0 * mz2;
            z = [
                (z[0] * z[0] - z[1] * z[1] - z[2] * z[2] - z[3] * z[3]) + power[0],
                (2.0 * z[0] * z[1]) + power[1],
                (2.0 * z[0] * z[2]) + power[2],
                (2.0 * z[0] * z[3]) + power[3],
            ];

            trap = [
                min_f64(trap[0], z[0].abs()),
                min_f64(trap[1], z[1].abs()),
                min_f64(trap[2], z[2].abs()),
                min_f64(trap[3], vecmath::vec4_dot(z, z)),
            ];

            mz2 = vecmath::vec4_dot(z, z);
            if mz2 > 4.0 {
                break;
            }
            n += 1.0
        }

        PrimitiveResult {
            distance: 0.25 * (mz2 / md2).sqrt() * mz2.log(std::f64::consts::E),
            fractal_data: trap,
            material: self.material_raw
        }
    }
    fn get_primitive_data(&self) -> &PrimitiveData {
        return &self.primtive_data;
    }
    fn get_primitive_data_mut(&mut self) -> &mut PrimitiveData {
        return &mut self.primtive_data;
    }
    fn _evaluate(&mut self, t: f64) {}
    fn _use_raw_position(&self) -> bool {
        return false;
    }
}
