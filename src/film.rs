use crate::configuration;
use crate::filter;
use crate::postprocessor;

pub trait Film{
    fn write_pixel(&mut self, x: u32, y: u32, col: [f64; 3]);
    fn save_image(&self, path: &str);
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

        let index = ((y*configuration::width+x) * 3) as usize;
        self.data[index] += new_col[0];
        self.data[index + 1] += new_col[1];
        self.data[index + 2] += new_col[2];
    }

    fn save_image(&self, path: &str){
        println!("Saving");
        let mut new_data = self.data.clone();
        for p in &self.post_processor{
            new_data = p.process_image(new_data);
        }
        println!("Enter it");
        let mut buff8: Vec<u8> = Vec::with_capacity((configuration::width * configuration::height * 3 * 2) as usize);
        for &float in new_data.iter() {
            let bytes = ((float * 256.0 * 256.0) as u16).to_le_bytes();
            buff8.push(bytes[0]);
            buff8.push(bytes[1]);
        }
        // let mut img: RgbImage = ImageBuffer::new(Config.width, Config.height);
        // for (x, y, pixel) in img.enumerate_pixels_mut() {
        //     let index = ((y*Config.width+x) * 3) as usize;
        //     let r = (new_data[index] * 255.0) as u8;
        //     let g = (new_data[index + 1] * 255.0) as u8;
        //     let b = (new_data[index + 2] * 255.0) as u8;
        //     *pixel = image::Rgb([r, g, b]);
        // }
        let res = image::save_buffer(path, &buff8, configuration::width, configuration::height, image::ColorType::Rgb16);
        match res {
            Ok(a) => {},
            Err(error) => {println!("{}", error.to_string())}
        }
        
    }
    fn evaluate(&mut self, t: f64){
        for i in &mut self.filter{
            i.evaluate(t);
        }
    }
    fn prepare_render(&mut self){
        let d = &mut self.data;
        for x in 0..configuration::width{
            for y in 0..configuration::height{
                let index = (y*configuration::width+x) * 3;
                d[index as usize] = 0.0;
                d[(index + 1) as usize] = 0.0;
                d[(index + 2) as usize] = 0.0;
            }
        }
    }
}
impl BasicFilm{
    pub fn new(filter: Vec<Box<dyn filter::Filter>>, post_processor: Vec<Box<dyn postprocessor::PostProcessor>>) -> BasicFilm{
        let index = configuration::width * configuration::height * 3;
        let mut data = Vec::<f64>::with_capacity(index as usize);

        for x in 0..configuration::width{
            for y in 0..configuration::height{
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