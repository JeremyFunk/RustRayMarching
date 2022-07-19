#![allow(non_upper_case_globals)]

pub const width: u32 = 720;
pub const height: u32 = 480;
pub const width_f: f64 = 720.0;
pub const height_f: f64 = 480.0;
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
pub const still_frame: u32 = 2;
pub const end_frame: u32 = ups as u32 * 4;
pub const start_frame: u32 = 0;
pub const threads: u32 = 23;
pub const samples: u32 = 1;

pub const render_scene: bool = true;
pub const video: bool = true;
