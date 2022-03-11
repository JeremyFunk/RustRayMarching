use std::{rc::Rc, cell::RefCell};
use std::sync::Mutex;
use crate::transition;
thread_local! {
    #[allow(non_upper_case_globals)]
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
        v.lock().unwrap().evaluate(t);
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
    pub fn new_get(value: f64) -> FloatEvaluator{
        FloatEvaluator{value}
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
    transition_function: Box<dyn transition::Transition>
}
impl Evaluator for InterpolatorEvaluator{
    fn evaluate(&self, t: f64) -> f64{
        let mut f = 0.0;
        if t % self.interval != 0.0 {
            f = (t % self.interval) / self.interval;
        }

        f = self.transition_function.transition(f);
        
        if self.oscilate && t % (self.interval * 2.0) >= self.interval{
            f = 1.0 - f;
        }
        self.min * (1.0 - f) + self.max * f
    }
}
impl InterpolatorEvaluator{
    pub fn new(min: f64, max: f64, interval: f64, oscilate: bool, transition_function: Box<dyn transition::Transition>, pointer: f64!()){
        let new_f = InterpolatorEvaluator{min, max, interval, oscilate, transition_function};
        insert_evaluator(pointer, Box::new(new_f));
    }
    pub fn new_get(min: f64, max: f64, interval: f64, oscilate: bool, transition_function: Box<dyn transition::Transition>) -> InterpolatorEvaluator{
        InterpolatorEvaluator{min, max, interval, oscilate, transition_function}
    }
}






pub struct CombineEvaluatorInfo{
    evaluator: Box<dyn Evaluator>,
    start: f64,
    stop: f64
}

impl CombineEvaluatorInfo{
    pub fn new(start: f64, stop: f64, evaluator: Box<dyn Evaluator>) -> CombineEvaluatorInfo{
        CombineEvaluatorInfo{start,stop,evaluator}
    }
}

pub struct CombineEvaluator{
    evaluators: Vec<CombineEvaluatorInfo>
}

impl Evaluator for CombineEvaluator{
    fn evaluate(&self, t: f64) -> f64{
        for e in &self.evaluators{
            if e.start <= t && e.stop >= t {
                return e.evaluator.evaluate(t - e.start)
            }
        }
        t
    }
}

impl CombineEvaluator{
    pub fn new(evaluators: Vec<CombineEvaluatorInfo>, pointer: f64!()){
        let new_f = CombineEvaluator{evaluators};
        insert_evaluator(pointer, Box::new(new_f));
    }
}