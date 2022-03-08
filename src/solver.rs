use std::borrow::BorrowMut;

use crate::primitive;
use crate::configuration::Config;
use vecmath;

pub struct IntersectionInfo{
    pub hit: bool,
    pub dist: f64,
    pub steps: u32,
    pub fractal_data: [f64;3],
    pub normal: [f64;3],
    pub position: [f64;3]
}

struct Solve{
    hit: bool,
    dist: f64,
    fractal_data: [f64;3],
    steps: u32,
    pos: [f64;3]
}

pub trait Solver {
    fn solve(&self, ray: crate::Ray) -> IntersectionInfo;
    fn evaluate(&mut self, t: f64);
}

pub struct GeneralSolver{
    primitives: Vec<Box<dyn primitive::Primitive>>
}

impl GeneralSolver{
    fn _calculate_normal(&self, pos: [f64;3]) -> [f64;3]{
        let gradient_x = self._map_world([pos[0] + Config.small_step, pos[1], pos[2]]).distance - self._map_world([pos[0] - Config.small_step, pos[1], pos[2]]).distance;
        let gradient_y = self._map_world([pos[0], pos[1] + Config.small_step, pos[2]]).distance - self._map_world([pos[0], pos[1] - Config.small_step, pos[2]]).distance;
        let gradient_z = self._map_world([pos[0], pos[1], pos[2] + Config.small_step]).distance - self._map_world([pos[0], pos[1], pos[2] - Config.small_step]).distance;
        return vecmath::vec3_normalized([gradient_x, gradient_y, gradient_z])
    }

    fn _map_world(&self, pos: [f64;3]) -> primitive::PrimitiveResult{
        let mut prim_data = primitive::PrimitiveResult{distance: 99999.0, fractal_data: [0.0, 0.0, 0.0]};
        for p in &self.primitives{
            let dist = p.map_primitive(pos);
            if(dist.distance < prim_data.distance){
                prim_data = dist;
            }
        }
        return prim_data
    }

    fn _solve_world(&self, pos: [f64;3], dir: [f64;3]) -> Solve{
        let mut mapped_pos = [pos[0], pos[1], pos[2]];
        let mut total_dist = 0.0;
        for i in 0..Config.step_number{
            let dist = self._map_world(mapped_pos);
            if(dist.distance < Config.min_dist){
                return Solve{hit: true, dist: total_dist, fractal_data: dist.fractal_data, steps: i, pos: mapped_pos}
            }
            if(dist.distance > Config.max_dist){
                return Solve{hit: false, dist: total_dist, fractal_data: dist.fractal_data, steps: i, pos: mapped_pos}
            }
            total_dist += dist.distance;
            mapped_pos = [mapped_pos[0] + dir[0] * dist.distance, mapped_pos[1] + dir[1] * dist.distance, mapped_pos[2] + dir[2] * dist.distance]
        }
        return Solve{hit: false, dist: total_dist, fractal_data: [0.0, 0.0, 0.0], steps: Config.step_number, pos: mapped_pos}
    }
}

impl Solver for GeneralSolver{
    fn solve(&self, ray: crate::Ray) -> IntersectionInfo{
        let res = self._solve_world(ray.1, ray.0);
        let normal = self._calculate_normal(res.pos);
        return IntersectionInfo{dist: res.dist, hit: res.hit, fractal_data: res.fractal_data, normal: normal, position: res.pos, steps: res.steps}
    }
    fn evaluate(&mut self, t: f64){
        for p in &mut self.primitives{
           p.evaluate(t);
        }
    }
}

impl GeneralSolver{
    pub fn new(primitives: Vec<Box<dyn primitive::Primitive>>) -> GeneralSolver{
        GeneralSolver{
            primitives
        }
    }
}