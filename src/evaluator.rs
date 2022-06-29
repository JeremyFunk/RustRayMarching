use std::{rc::Rc, cell::RefCell};
use std::sync::Mutex;
use crate::transition;
use crate::helpers;
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


pub struct Keyframe{
    pub frame: f64,
    pub value: f64,
    pub inter_x_in: f64,
    pub inter_x_out: f64,
    pub inter_y_in: f64,
    pub inter_y_out: f64,
}

impl Keyframe{
    pub fn new(frame: f64, value: f64, inter_x_in: f64, inter_x_out: f64, inter_y_in: f64, inter_y_out: f64) -> Keyframe{
        Keyframe { frame: frame, value: value, inter_x_in: inter_x_in, inter_x_out: inter_x_out, inter_y_in: inter_y_in, inter_y_out: inter_y_out }
    }
}

pub struct KeyframeEvaluator{
    keyframes: Vec<Keyframe>,
}


impl Evaluator for KeyframeEvaluator{

    fn evaluate(&self, t: f64) -> f64{
        for (i, k) in self.keyframes.iter().enumerate(){
            if t < k.frame {
                if i == 0 {
                    return k.value
                }else{
                    let low = &self.keyframes[i-1];

                    let f_diff = (k.frame - low.frame);
                    let f = (t - low.frame) / f_diff;
                    let res = helpers::im_bezier_cubic_calc(0.0, low.value, low.inter_x_out / f_diff, low.value + low.inter_y_out, 1.0 + k.inter_x_in / f_diff, k.value + k.inter_y_in, 1.0, k.value, f);
                    //return low.value * (1.0 - f) + k.value * f;
                    return res[1];
                }
            }
        }
        self.keyframes[self.keyframes.len() - 1].value
    }
}

impl KeyframeEvaluator{
    pub fn new(keyframes: Vec<Keyframe>, pointer: f64!()){
        let new_f = KeyframeEvaluator{keyframes};
        insert_evaluator(pointer, Box::new(new_f));
    }
    pub fn new_get(value: f64, keyframes: Vec<Keyframe>) -> KeyframeEvaluator{
        KeyframeEvaluator{keyframes}
    }
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


