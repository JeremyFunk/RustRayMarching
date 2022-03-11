use crate::configuration::Config;
use crate::filter;
use crate::postprocessor;
use image::{ImageBuffer, RgbImage};

pub trait Film{
    fn write_pixel(&mut self, x: u32, y: u32, col: [f64; 3]);
    fn build_image(&self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;
    fn evaluate(&mut self, t: f64);
    fn prepare_render(&mut self);
}

pub struct BasicFilm {
    data: Vec<f64>,
    filter: Vec<Box<dyn filter::Filter>>,
    post_processor: Vec<Box<dyn postprocessor::PostProcessor>>
}

impl Film for BasicFilm{
    fn write_pixel(&mut self, x: u32, y: u32, col: [f64; 3]){
        let mut new_col = [col[0], col[1], col[2]];
        for i in &self.filter{
            new_col = i.filter_color(x, y, new_col);
        }

        let index = ((y*Config.width+x) * 3) as usize;
        self.data[index] = new_col[0];
        self.data[index + 1] = new_col[1];
        self.data[index + 2] = new_col[2];
    }

    fn build_image(&self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>{
        let mut new_data = self.data.clone();
        for p in &self.post_processor{
            new_data = p.process_image(new_data);
        }

        let mut img: RgbImage = ImageBuffer::new(Config.width, Config.height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let index = ((y*Config.width+x) * 3) as usize;
            let r = (new_data[index] * 255.0) as u8;
            let g = (new_data[index + 1] * 255.0) as u8;
            let b = (new_data[index + 2] * 255.0) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        return img;
    }
    fn evaluate(&mut self, t: f64){
        for i in &mut self.filter{
            i.evaluate(t);
        }
    }
    fn prepare_render(&mut self){
        let d = &mut self.data;
        for x in 0..Config.width{
            for y in 0..Config.height{
                let index = (y*Config.width+x) * 3;
                d[index as usize] = 0.0;
                d[(index + 1) as usize] = 0.0;
                d[(index + 2) as usize] = 0.0;
            }
        }
    }
}
impl BasicFilm{
    pub fn new(filter: Vec<Box<dyn filter::Filter>>, post_processor: Vec<Box<dyn postprocessor::PostProcessor>>) -> BasicFilm{
        let index = Config.width * Config.height * 3;
        let mut data = Vec::<f64>::with_capacity(index as usize);

        for x in 0..Config.width{
            for y in 0..Config.height{
                data.push(0.0);
                data.push(0.0);
                data.push(0.0);
            }
        }
        BasicFilm{
            data,
            filter,
            post_processor
        }
    }
}