#![allow(non_upper_case_globals)]

pub const width: u32 = 1920;
pub const height: u32 = 1080;
pub const width_f: f64 = 1920.0;
pub const height_f: f64 = 1080.0;
pub const fov: f64 = 90.0;
pub const step_number: u32 = 256;
pub const light_step_number: u32 = 256;
pub const light_min_dist: f64 = 0.001;
pub const min_dist: f64 = 0.001;
pub const max_dist: f64 = 300.0;
pub const small_step: f64 = 0.001;
pub const num_group_modifiers: usize = 4;
pub const fps: u32 = 30;
pub const ups: f64 = 30.0;
pub const still_frame: u32 = 45;
pub const end_frame: u32 = ups as u32 * 5;
pub const start_frame: u32 = 0;
pub const threads: u32 = 16;
pub const samples: u32 = 2;
pub const render_scene: bool = true;
pub const video: bool = true;
