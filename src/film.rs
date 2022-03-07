use crate::configuration::Config;
use image::{ImageBuffer, RgbImage};

pub trait Film{
    fn write_pixel(&mut self, x: u32, y: u32, col: [f64; 3]);
    fn build_image(&self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;
}

pub struct BasicFilm {
    data: Vec<f64>
}

impl Film for BasicFilm{
    fn write_pixel(&mut self, x: u32, y: u32, col: [f64; 3]){
        let index = ((x * Config.width + y) * 3) as usize;
        self.data[index] = col[0];
        self.data[index + 1] = col[1];
        self.data[index + 2] = col[2];
    }

    fn build_image(&self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>{
        let mut img: RgbImage = ImageBuffer::new(Config.width, Config.height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let index = ((x * Config.width + y) * 3) as usize;
            let r = (self.data[index] * 255.0) as u8;
            let g = (self.data[index + 1] * 255.0) as u8;
            let b = (self.data[index + 2] * 255.0) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        return img;
    }
}
impl BasicFilm{
    pub fn new() -> BasicFilm{
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
            data
        }
    }
}