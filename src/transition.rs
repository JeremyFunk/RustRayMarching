pub trait Transition{
    fn transition(&self, t: f64) -> f64;
}


pub struct Linear {
}

impl Transition for Linear{
    fn transition(&self, t: f64) -> f64{
        return t
    }
}

impl Linear{
    pub fn new() -> Linear{
        Linear{
        }
    }
}

pub struct Smoothstep {
    factor: f64,
}

impl Transition for Smoothstep{
    fn transition(&self, t: f64) -> f64{
        if self.factor == 1.0 {
            return t * t * (3.0 - 2.0 * t)
        }
        if self.factor == 2.0 {
            return t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
        }
        else
        {
            let xsq = t * t; 
            let xsqsq = xsq * xsq; 
            return xsqsq * (25.0 - 48.0 * t + xsq * (25.0 - xsqsq));
        }
    }
}

impl Smoothstep{
    pub fn new(factor: f64) -> Smoothstep{
        Smoothstep{
            factor
        }
    }
}




pub struct CatmullRomSpline {
    p0: f64,
    p3: f64,
}
impl Transition for CatmullRomSpline{
    fn transition(&self, t: f64) -> f64{
        return self.catmull_rom_spline(t, self.p0, 0.0, 1.0, self.p3);
    }

}

impl CatmullRomSpline{
    pub fn new(p0: f64, p3: f64) -> CatmullRomSpline{
        CatmullRomSpline{
            p0, p3
        }
    }
    fn catmull_rom_spline(&self, t: f64, p0: f64, p1: f64, p2: f64, p3: f64) -> f64{
        return 0.5 * ((2.0 * p1) + (-p0 + p2) * t + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t * t * t)
    }
}
