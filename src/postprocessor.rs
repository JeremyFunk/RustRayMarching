use std::borrow::{BorrowMut, Borrow};
use std::{rc::Rc, cell::RefCell};
use crate::helpers;
use crate::configuration::Config;
pub trait PostProcessor{
    fn process_image(&self, data: Vec<f64>) -> Vec<f64>;
    fn evaluate(&self, t:f64);
}

fn cut_image(cut: f64, data: &Vec<f64>) -> Vec<[f64;3]>{
    let mut u: Vec<[f64;3]> = Vec::new();
    for o in 0..(data.len() / 3){
        u.push([
            if data[o*3]     > cut {data[o*3]}     else {0.0},
            if data[o*3 + 1] > cut {data[o*3 + 1]} else {0.0},
            if data[o*3 + 2] > cut {data[o*3 + 2]} else {0.0}
        ])
    }
    u
}

fn convert_image(data: &Vec<f64>) -> Vec<[f64;3]>{
    let mut u: Vec<[f64;3]> = Vec::new();
    for o in 0..(data.len() / 3){
        u.push([
            data[o*3],  
            data[o*3 + 1] ,
            data[o*3 + 2]
        ])
    }
    u
}
pub struct BloomPostProcessor{
    bloom_cut: f64!(),
    factor: f64!(),
    size: f64!()
}


impl PostProcessor for BloomPostProcessor{
    fn process_image(&self, data: Vec<f64>) -> Vec<f64>{
        let factor = get_ref_f64!(self.factor);
        let data_bor: &Vec<f64> = data.borrow();
        let mut cut_img = cut_image(get_ref_f64!(self.bloom_cut), data_bor);
        helpers::gaussian_blur(cut_img.borrow_mut(), Config.width as usize, Config.height as usize, get_ref_f64!(self.size));
        let mut d: Vec<f64> = Vec::new();
        for i in 0..cut_img.len(){
            d.push(
                cut_img[i][0] * factor + data_bor[i * 3]
            );
            d.push(
                cut_img[i][1] * factor + data_bor[i * 3 + 1]
            );
            d.push(
                cut_img[i][2] * factor + data_bor[i * 3 + 2]
            );
        }
        d
    }
    fn evaluate(&self, t:f64){

    }
}

impl BloomPostProcessor{
    pub fn new(bloom_cut: f64!(), factor: f64!(), size: f64!()) -> BloomPostProcessor{
        BloomPostProcessor{bloom_cut, factor, size}
    }
}






pub struct BlurPostProcessor{
    size: f64!()
}


impl PostProcessor for BlurPostProcessor{
    fn process_image(&self, data: Vec<f64>) -> Vec<f64>{
        let data_bor: &Vec<f64> = data.borrow();
        let mut cut_img = convert_image(data_bor);
        helpers::gaussian_blur(cut_img.borrow_mut(), Config.width as usize, Config.height as usize, get_ref_f64!(self.size));
        let mut d: Vec<f64> = Vec::new();
        for i in 0..cut_img.len(){
            d.push(
                cut_img[i][0]
            );
            d.push(
                cut_img[i][1]
            );
            d.push(
                cut_img[i][2]
            );
        }
        d
    }
    fn evaluate(&self, t:f64){

    }
}

impl BlurPostProcessor{
    pub fn new(size: f64!()) -> BlurPostProcessor{
        BlurPostProcessor{size}
    }
}
