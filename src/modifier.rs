use crate::configuration::Config;
use crate::helpers;

pub trait PosModifier{
    fn modify(&self, pos: [f64; 3]) -> [f64; 3];
    fn evaluate(&self, t: f64);
}

pub trait DistModifier{
    fn modify(&self, dist: f64) -> f64;
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
}








pub struct Twist {
    power: f64,
}

impl PosModifier for Twist{
    fn modify(&self, pos: [f64; 3]) -> [f64; 3]{
        let c = (self.power * pos[1]).cos();
        let s = (self.power * pos[1]).sin();
        return [c * pos[0] - s * pos[2], s * pos[0] + c * pos[2], pos[1]];
    }
    fn evaluate(&self, t: f64){

    }
}

impl Twist{
    pub fn new(power: f64) -> Twist{
        Twist{
            power
        }
    }
}











pub struct Bend {
    power: f64,
}

impl PosModifier for Bend{
    fn modify(&self, pos: [f64; 3]) -> [f64; 3]{
        let c = (self.power * pos[0]).cos();
        let s = (self.power * pos[0]).sin();
        return [c * pos[0] - s * pos[1], s * pos[0] + c * pos[1], pos[2]];
    }
    fn evaluate(&self, t: f64){

    }
}

impl Bend{
    pub fn new(power: f64) -> Bend{
        Bend{
            power
        }
    }
}











pub struct Repetition {
    repetition_period: f64
}

impl PosModifier for Repetition{
    fn modify(&self, pos: [f64; 3]) -> [f64; 3]{
        return [
            ((pos[0] + 0.5 * self.repetition_period) % self.repetition_period) - 0.5 * self.repetition_period,
            ((pos[1] + 0.5 * self.repetition_period) % self.repetition_period) - 0.5 * self.repetition_period,
            ((pos[2] + 0.5 * self.repetition_period) % self.repetition_period) - 0.5 * self.repetition_period
        ]
    }
    
    fn evaluate(&self, t: f64){

    }
}

impl Repetition{
    pub fn new(repetition_period: f64) -> Repetition{
        Repetition{
            repetition_period
        }
    }
}












pub struct RepetitionLimited {
    repetition_period: f64,
    limiter: [f64;3]
}

impl PosModifier for RepetitionLimited{
    fn modify(&self, pos: [f64; 3]) -> [f64; 3]{
        return [
            (pos[0] - self.repetition_period * helpers::round(pos[0] / self.repetition_period).clamp(-self.limiter[0], self.limiter[0])),
            (pos[1] - self.repetition_period * helpers::round(pos[1] / self.repetition_period).clamp(-self.limiter[1], self.limiter[1])),
            (pos[2] - self.repetition_period * helpers::round(pos[2] / self.repetition_period).clamp(-self.limiter[2], self.limiter[2]))
        ];
    }
    fn evaluate(&self, t: f64){

    }
}

impl RepetitionLimited{
    pub fn new(repetition_period: f64, limiter: [f64;3]) -> RepetitionLimited{
        RepetitionLimited{
            repetition_period,
            limiter,
        }
    }
}













pub struct Round {
    thickness: f64,
}


impl DistModifier for Round{
    fn modify(&self, dist: f64) -> f64{
        return dist.abs() - self.thickness;
    }
    fn evaluate(&self, t: f64){

    }
}

impl Round{
    pub fn new(thickness: f64) -> Round{
        Round{
            thickness,
        }
    }
}

