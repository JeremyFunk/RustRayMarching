use crate::{configuration, CameraSample};

pub trait Sampler{
    fn generate_samples(&self, x: u32, y: u32) -> [CameraSample;(configuration::samples*configuration::samples) as usize];
    fn evaluate(&mut self, t: f64);
}

pub struct JitterSampler{
    jitter: f64,
    total_jitter: f64
}
const SAMPLE_INV: f64 = 1.0 / (configuration::samples as f64);

impl Sampler for JitterSampler{
    fn generate_samples(&self, x: u32, y: u32) -> [CameraSample;(configuration::samples*configuration::samples) as usize]{
        let mut elements = [((0.0, 0.0),(0.0, 0.0));(configuration::samples*configuration::samples) as usize];
        
        for sx in 0..configuration::samples{
            for sy in 0..configuration::samples{
                let film_sample = (sx as f64 * SAMPLE_INV + ((fastrand::f64() - 0.5) * self.total_jitter), sy as f64 * SAMPLE_INV + ((fastrand::f64() - 0.5) * self.total_jitter));
                let lens_sample = (sx as f64 * SAMPLE_INV + ((fastrand::f64() - 0.5) * self.total_jitter), sy as f64 * SAMPLE_INV + ((fastrand::f64() - 0.5) * self.total_jitter));
                elements[(sx * configuration::samples + sy) as usize] = (film_sample, lens_sample);
            }
        }
        return elements
    }
    fn evaluate(&mut self, t: f64){
        
    }
}

impl JitterSampler{
    pub fn new(jitter: f64) -> JitterSampler{
        JitterSampler{jitter, total_jitter: jitter * SAMPLE_INV}
    }
}