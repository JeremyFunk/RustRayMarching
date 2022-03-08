use lazy_static::lazy_static;

lazy_static! {
    pub static ref Config: Configuration = Configuration::new();
}

pub struct Configuration{
    pub width: u32,
    pub height: u32,
    pub width_f: f64,
    pub height_f: f64,
    pub fov: f64,
    pub step_number: u32,
    pub min_dist: f64,
    pub max_dist: f64,
    pub small_step: f64,
}

impl Configuration{
    pub fn new() -> Configuration{

        Configuration{
            width: 512, 
            height: 512, 
            width_f: 512.0, 
            height_f: 512.0, 
            fov: 90.0,
            step_number: 64,
            min_dist: 0.001,
            max_dist: 300.0,
            small_step: 0.001
        }
    }
}