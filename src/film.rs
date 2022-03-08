use crate::configuration::Config;
use crate::filter;
use image::{ImageBuffer, RgbImage};

pub trait Film{
    fn write_pixel(&mut self, x: u32, y: u32, col: [f64; 3]);
    fn build_image(&self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;
}

pub struct BasicFilm<F: filter::Filter> {
    data: Vec<f64>,
    filter: F
}

impl<F: filter::Filter> Film for BasicFilm<F>{
    fn write_pixel(&mut self, x: u32, y: u32, col: [f64; 3]){
        let new_col = self.filter.filter_color(x, y, col);

        let index = ((x * Config.width + y) * 3) as usize;
        self.data[index] = new_col[0];
        self.data[index + 1] = new_col[1];
        self.data[index + 2] = new_col[2];
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
impl<F: filter::Filter> BasicFilm<F>{
    pub fn new(filter: F) -> BasicFilm<F>{
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
            filter
        }
    }
}