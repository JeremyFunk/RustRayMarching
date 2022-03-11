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
    pub video: bool,
    pub still_frame: u32,
    pub fps: u32,
    pub ups: f64,
    pub end_frame: u32,
    pub start_frame: u32,
    pub threads: u32,
}

impl Configuration{
    pub fn new() -> Configuration{
        let ups = 96;
        Configuration{
            width: 1920, 
            height: 1280, 
            width_f: 1920.0, 
            height_f: 1280.0, 
            fov: 90.0,
            step_number: 64,
            min_dist: 0.001,
            max_dist: 300.0,
            small_step: 0.001,
            video: true,
            still_frame: ups * 3,
            fps: 24,
            ups: ups as f64,
            end_frame: ups * 6,
            start_frame: 0,
            threads: 16,
        }
    }
}