use crate::configuration::Config;

pub trait PosModifier{
    fn modify(&self, pos: [f64; 3]) -> [f64; 3];
    fn evaluate(&self, t: f64);
}

pub struct Distort {
    factor: f64,
    offset: [f64;3],
    freq: f64
}

impl PosModifier for Distort{
    fn modify(&self, pos: [f64; 3]) -> [f64; 3]{
        let m = (self.freq * pos[0] + self.offset[0]).sin() * (self.freq * pos[1] + self.offset[1]).sin() * (self.freq * pos[2] + self.offset[2]).sin() * self.factor;
        return [pos[0] + m, pos[1] + m, pos[2] + m];
    }
    fn evaluate(&self, t: f64){
        
    }
}

impl Distort{
    pub fn new(factor: f64, offset: [f64;3], freq: f64) -> Distort{
        Distort{
            factor,offset,freq
        }
    }
    fn evaluate(&self, t: f64){
        
    }
}