use std::{rc::Rc, cell::RefCell};
use std::sync::Mutex;
thread_local! {
    static manager: Mutex<EvaluatorManager> = Mutex::new(EvaluatorManager{evaluators: Vec::new()});
}

fn insert_evaluator(pointer: f64!(), evaluator: Box<dyn Evaluator>){
    manager.with(|v| {
        let mut eval = v.lock().unwrap();
        let eval_p = EvaluatorPointer{pointer, evaluator};
        eval.evaluators.push(eval_p);
    });
}

pub fn evaluate(t: f64){
    manager.with(|v| {
        let mut eval = v.lock().unwrap().evaluate(t);
    });
}

struct EvaluatorPointer{
    pointer: f64!(),
    evaluator: Box<dyn Evaluator>
}
pub struct EvaluatorManager{
    evaluators: Vec<EvaluatorPointer>
}
impl EvaluatorManager{
    pub fn evaluate(&self, t: f64){
        for a in &self.evaluators{
            a.pointer.replace(a.evaluator.evaluate(t));
        }
    }
}



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
    pub fn new(value: f64, pointer: f64!()){
        let new_f = FloatEvaluator{value};
        insert_evaluator(pointer, Box::new(new_f));
    }
}

pub fn convert_to_evaluator(a: [f64;3], pointer: f64v!()){
    FloatEvaluator::new(a[0], pointer[0].clone());
    FloatEvaluator::new(a[1], pointer[1].clone());
    FloatEvaluator::new(a[2], pointer[2].clone());
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
        println!("{}", self.min * (1.0 - f) + self.max * f);
        self.min * (1.0 - f) + self.max * f
    }
}
impl InterpolatorEvaluator{
    pub fn new(min: f64, max: f64, interval: f64, oscilate: bool, pointer: f64!()){
        let new_f = InterpolatorEvaluator{min, max, interval, oscilate};
        insert_evaluator(pointer, Box::new(new_f));
    }
}