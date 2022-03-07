use crate::configuration::Config;

pub trait Camera{
    fn generate_ray(&self, x: f64, y: f64) -> crate::Ray;
}

pub struct PinholeCamera{
    pos: [f64; 3],
    rot: [f64; 3],
    ar: f64,
    scale: f64
}

impl Camera for PinholeCamera{
    fn generate_ray(&self, x: f64, y: f64) -> crate::Ray{
        let rx = (2.0 * (x + 0.5) / Config.width_f - 1.0) * self.ar * self.scale;
        let ry = (1.0 - 2.0 * (y + 0.5) / Config.height_f) * self.scale;

        return (vecmath::vec3_normalized([rx, ry, -1.0]), [self.pos[0], self.pos[1],self.pos[2]])
    }
}

impl PinholeCamera{
    pub fn new(pos: [f64; 3], rot: [f64; 3]) -> PinholeCamera{
        let w_f = Config.width as f64;
        let h_f = Config.height as f64;
        let ar = w_f/h_f;
        let scale = (Config.fov * 0.5).to_radians();
        PinholeCamera{
            pos, 
            rot, 
            ar,
            scale
        }
    }
}