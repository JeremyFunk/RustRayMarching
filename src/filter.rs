use std::{rc::Rc, cell::RefCell};

pub trait Filter{
    fn filter_color(&self, x: u32, y: u32, col: [f64; 3]) -> [f64;3];
    fn evaluate(&mut self, t: f64);
}

pub enum ColorShiftMode{
    Mul,
    Div,
    Add,
    Sub
}

pub struct ColorShift {
    color: [f64!();3],
    mode: ColorShiftMode
}

impl Filter for ColorShift{
    fn filter_color(&self, x: u32, y: u32, col: [f64; 3]) -> [f64;3]{
        let color = get_f64v!(self.color);
        if matches!(self.mode, ColorShiftMode::Add){
            return [col[0] + color[0], col[1] + color[1], col[2] + color[2]]
        }
        if matches!(self.mode, ColorShiftMode::Sub){
            return [col[0] - color[0], col[1] - color[1], col[2] - color[2]]
        }
        if matches!(self.mode, ColorShiftMode::Mul){
            return [col[0] * color[0], col[1] * color[1], col[2] * color[2]]
        }
        if matches!(self.mode, ColorShiftMode::Div){
            return [col[0] / color[0], col[1] / color[1], col[2] / color[2]]
        }
        return col;
    }
    fn evaluate(&mut self, t: f64){
        
    }
}
impl ColorShift{
    pub fn new(color: [f64!(); 3], mode: ColorShiftMode) -> ColorShift{
        ColorShift{
            color,
            mode
        }
    }
}




pub struct GrayFilter {
    strength: f64!()
}

impl Filter for GrayFilter{
    fn filter_color(&self, x: u32, y: u32, col: [f64; 3]) -> [f64;3]{
        let strength = get_f64!(self.strength);
        let strength_inv = 1.0 - strength;
        // println!("{}", strength_inv);
        let brightness = (col[0] + col[1] + col[2]) * 0.33333 * strength;
        [
            col[0] * strength_inv + brightness,
            col[1] * strength_inv + brightness,
            col[2] * strength_inv + brightness,
        ]
    }
    fn evaluate(&mut self, t: f64){
        
    }
}
impl GrayFilter{
    pub fn new(strength: f64!()) -> GrayFilter{
        GrayFilter{
            strength
        }
    }
}