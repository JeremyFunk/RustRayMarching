use crate::configuration;

pub trait Sampler{
    fn generate_samples(&self, x: u32, y: u32) -> [[f64;2];(configuration::samples*configuration::samples) as usize];
    fn evaluate(&mut self, t: f64);
}

pub struct JitterSampler{
    jitter: f64
}
const SAMPLE_INV: f64 = 1.0 / (configuration::samples as f64);

impl Sampler for JitterSampler{
    fn generate_samples(&self, x: u32, y: u32) -> [[f64;2];(configuration::samples*configuration::samples) as usize]{
        let mut elements = [[0.0, 0.0];(configuration::samples*configuration::samples) as usize];
        
        for sx in 0..configuration::samples{
            for sy in 0..configuration::samples{
                elements[(sx * configuration::samples + sy) as usize] = [sx as f64 * SAMPLE_INV + ((fastrand::f64() - 0.5) * self.jitter * SAMPLE_INV), sy as f64 * SAMPLE_INV + ((fastrand::f64() - 0.5) * self.jitter * SAMPLE_INV)];
            }
        }
        return elements
    }
    fn evaluate(&mut self, t: f64){
        
    }
}

impl JitterSampler{
    pub fn new(jitter: f64) -> JitterSampler{
        JitterSampler{jitter}
    }
}