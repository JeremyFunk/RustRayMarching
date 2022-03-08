
pub trait Evaluator{
    fn evaluate(&self, t: f64) -> f64;
}
pub struct FloatEvaluator{
    value: f64
}
impl Evaluator for FloatEvaluator{
    fn evaluate(&self, t: f64) -> f64{
        self.value
    }
}
impl FloatEvaluator{
    pub fn new(value: f64) -> FloatEvaluator{
        FloatEvaluator{value}
    }
}

pub fn convert_to_evaluator(a: [f64;3]) -> [Box<dyn Evaluator>;3]{
    return [
        Box::new(FloatEvaluator::new(a[0])),
        Box::new(FloatEvaluator::new(a[1])),
        Box::new(FloatEvaluator::new(a[2]))
    ]
}


pub struct InterpolatorEvaluator{
    min: f64,
    max: f64,
    interval: f64,
    oscilate: bool,
}
impl Evaluator for InterpolatorEvaluator{
    fn evaluate(&self, t: f64) -> f64{
        let mut f = 0.0;
        if t != 0.0 {
            f = (t % self.interval) / self.interval;
        }
        
        if self.oscilate && t % (self.interval * 2.0) >= self.interval{
            f = 1.0 - f;
        }
        
        self.min * (1.0 - f) + self.max * f
    }
}
impl InterpolatorEvaluator{
    pub fn new(min: f64, max: f64, interval: f64, oscilate: bool) -> InterpolatorEvaluator{
        InterpolatorEvaluator{min, max, interval, oscilate}
    }
}