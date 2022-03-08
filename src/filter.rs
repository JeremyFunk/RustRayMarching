use crate::configuration::Config;

pub trait Filter{
    fn filter_color(&self, x: u32, y: u32, col: [f64; 3]) -> [f64;3];
}

pub enum ColorShiftMode{
    Mul,
    Div,
    Add,
    Sub
}

pub struct ColorShift {
    color: [f64;3],
    mode: ColorShiftMode
}

impl Filter for ColorShift{
    fn filter_color(&self, x: u32, y: u32, col: [f64; 3]) -> [f64;3]{
        if(matches!(self.mode, ColorShiftMode::Add)){
            return [col[0] + self.color[0], col[1] + self.color[1], col[2] + self.color[2]]
        }
        if(matches!(self.mode, ColorShiftMode::Sub)){
            return [col[0] - self.color[0], col[1] - self.color[1], col[2] - self.color[2]]
        }
        if(matches!(self.mode, ColorShiftMode::Mul)){
            return [col[0] * self.color[0], col[1] * self.color[1], col[2] * self.color[2]]
        }
        if(matches!(self.mode, ColorShiftMode::Div)){
            return [col[0] / self.color[0], col[1] / self.color[1], col[2] / self.color[2]]
        }
        return col;
    }
}
impl ColorShift{
    pub fn new(color: [f64; 3], mode: ColorShiftMode) -> ColorShift{
        ColorShift{
            color,
            mode
        }
    }
}