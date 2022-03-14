#![allow(non_upper_case_globals)]

pub const width: u32 = 5120;
pub const height: u32 = 2880;
pub const width_f: f64 = 5120.0;
pub const height_f: f64 = 2880.0;
pub const fov: f64 = 90.0;
pub const step_number: u32 = 64;
pub const min_dist: f64 = 0.001;
pub const max_dist: f64 = 300.0;
pub const small_step: f64 = 0.001;
pub const video: bool = false;
pub const fps: u32 = 24;
pub const ups: f64 = 48.0;
pub const still_frame: u32 = 60;
pub const end_frame: u32 = ups as u32 * 6;
pub const start_frame: u32 = 0;
pub const threads: u32 = 16;
pub const samples: u32 = 2;
