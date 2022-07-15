use crate::{
    configuration,
    evaluator::{self, Evaluator},
};
use rhai::{Array, Engine, Func};
use serde::Deserialize;
use std::fs;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde()]
pub struct JsonAnimationKeyframe {
    pub frame: u32,
    pub value: f64,
    pub inter_x_in: f64,
    pub inter_x_out: f64,
    pub inter_y_in: f64,
    pub inter_y_out: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde()]
pub struct JsonAnimationFloat {
    pub keyframes: Option<Vec<JsonAnimationKeyframe>>,
    pub value: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde()]
pub struct JsonSceneModifier {
    pub attribute0: JsonAnimationFloat,
    pub attribute1: JsonAnimationFloat,
    pub attribute2: JsonAnimationFloat,
    pub attribute3: JsonAnimationFloat,
    pub attribute4: JsonAnimationFloat,
    pub modifier: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde()]
pub struct JsonSceneGroupModifier {
    pub prim0: i32,
    pub prim1: i32,
    pub prim2: i32,
    pub prim3: i32,
    pub prim_attribute: JsonAnimationFloat,
    pub modifier: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde()]
pub struct JsonSceneObject {
    pub position: [JsonAnimationFloat; 3],
    pub rotation: [JsonAnimationFloat; 3],
    pub scale: [JsonAnimationFloat; 3],
    pub prim_type: i32,
    pub values: [JsonAnimationFloat; 10],
    pub modifiers: std::vec::Vec<JsonSceneModifier>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde()]
pub struct JsonCamera {
    pub cam_pos: [JsonAnimationFloat; 3],
    pub cam_py: [JsonAnimationFloat; 2],
}

#[derive(Debug, Deserialize, Clone)]
#[serde()]
pub struct JsonScene {
    pub objects: std::vec::Vec<JsonSceneObject>,
    pub group_modifiers: std::vec::Vec<JsonSceneGroupModifier>,
    pub camera: JsonCamera,
}

pub struct SceneModifier {
    pub attribute0: f64!(),
    pub attribute1: f64!(),
    pub attribute2: f64!(),
    pub attribute3: f64!(),
    pub attribute4: f64!(),
    pub modifier: i32,
}

pub struct SceneGroupModifier {
    pub prim0: i32,
    pub prim1: i32,
    pub prim2: i32,
    pub prim3: i32,
    pub prim_attribute: f64!(),
    pub modifier: i32,
}

pub struct SceneObject {
    pub position: [f64!(); 3],
    pub rotation: [f64!(); 3],
    pub scale: [f64!(); 3],
    pub prim_type: i32,
    pub values: [f64!(); 10],
    pub modifiers: std::vec::Vec<SceneModifier>,
}

pub struct Camera {
    pub cam_pos: [f64!(); 3],
    pub cam_py: [f64!(); 2],
}
pub struct Scene {
    pub objects: std::vec::Vec<SceneObject>,
    pub group_modifiers: std::vec::Vec<SceneGroupModifier>,
    pub camera: Camera,
}

fn convert_animated_float(f: JsonAnimationFloat) -> f64!() {
    match f.keyframes {
        Some(p) => {
            if p.len() == 1 {
                return f64!(p[0].value);
            } else {
                let mut b: Vec<evaluator::Keyframe> = Vec::new();
                let f_type = f64!(p[0].value);
                for n in p {
                    let frame: f64 = n.frame.into();
                    let combine = evaluator::Keyframe::new(
                        frame / configuration::ups,
                        n.value,
                        n.inter_x_in,
                        n.inter_x_out,
                        n.inter_y_in,
                        n.inter_y_out,
                    );
                    b.push(combine);
                }
                evaluator::KeyframeEvaluator::new(b, f_type.clone());
                return f_type;
            }
        }
        None => {
            return f64!(f.value);
        }
    }
}

fn convert_offset(f: JsonAnimationFloat, add: f64, mul: f64) -> f64!() {
    match f.keyframes {
        Some(p) => {
            if p.len() == 1 {
                return f64!((p[0].value + add) * mul);
            } else {
                let mut b: Vec<evaluator::Keyframe> = Vec::new();
                let f_type = f64!((p[0].value + add) * mul);
                for n in p {
                    let frame: f64 = n.frame.into();
                    let combine = evaluator::Keyframe::new(
                        frame / configuration::ups,
                        (n.value + add) * mul,
                        n.inter_x_in,
                        n.inter_x_out,
                        n.inter_y_in,
                        n.inter_y_out,
                    );
                    b.push(combine);
                }
                evaluator::KeyframeEvaluator::new(b, f_type.clone());
                return f_type;
            }
        }
        None => {
            return f64!(f.value);
        }
    }
}

fn convert_script() -> [f64!(); 10] {
    let engine = Engine::new();
    let script = "fn evaluate(t) {
        t *= 0.15;

        return [0.45 * cos(0.5 + t * 1.2) - 0.3, 0.45 * cos(3.9 + t * 1.7), 0.45 * cos(1.4 + t * 1.3), 0.45 * cos(1.1 + t * 2.5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    }";
    let func = Func::<(f64,), Array>::create_from_script(engine, script, "evaluate");

    let func_res = match func {
        Ok(res) => res,
        Err(error) => panic!("Problem loading script: {:?}", error),
    };
    let values = [
        f64!(0.0),
        f64!(0.0),
        f64!(0.0),
        f64!(0.0),
        f64!(0.0),
        f64!(0.0),
        f64!(0.0),
        f64!(0.0),
        f64!(0.0),
        f64!(0.0),
    ];

    for j in 0..10 {
        let mut b: Vec<evaluator::Keyframe> = Vec::new();
        for i in configuration::start_frame..configuration::end_frame {
            let t = i as f64 / configuration::ups;

            let result = match func_res(t) {
                Ok(res) => res,
                Err(error) => panic!("Problem loading script: {:?}", error),
            };
            let frame: f64 = i.into();
            let combine = evaluator::Keyframe::new(
                frame / configuration::ups,
                result[j].clone().cast::<f64>(),
                0.0,
                0.0,
                0.0,
                0.0,
            );
            b.push(combine);
        }
        evaluator::KeyframeEvaluator::new(b, values[j].clone());
    }
    return values;
}

pub fn load_scene(path: &str) -> Scene {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let json: JsonScene = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    let camera: Camera = Camera {
        cam_pos: [
            convert_animated_float(json.camera.cam_pos[0].to_owned()),
            convert_animated_float(json.camera.cam_pos[1].to_owned()),
            convert_animated_float(json.camera.cam_pos[2].to_owned()),
        ],
        cam_py: [
            convert_offset(json.camera.cam_py[0].to_owned(), 0.0, -1.0),
            convert_offset(json.camera.cam_py[1].to_owned(), 90.0, -1.0),
        ],
    };
    //[-scene.camera.cam_py[0].value, -(scene.camera.cam_py[1].value+90.0), 0.0])

    let mut group_modifiers: std::vec::Vec<SceneGroupModifier> = Vec::new();
    for g in json.group_modifiers {
        group_modifiers.push(SceneGroupModifier {
            prim0: g.prim0,
            prim1: g.prim1,
            prim2: g.prim2,
            prim3: g.prim3,
            prim_attribute: convert_animated_float(g.prim_attribute),
            modifier: g.modifier,
        });
    }

    let mut objects: std::vec::Vec<SceneObject> = Vec::new();
    for o in json.objects {
        let mut mods: Vec<SceneModifier> = Vec::new();
        for m in o.modifiers {
            mods.push(SceneModifier {
                attribute0: convert_animated_float(m.attribute0),
                attribute1: convert_animated_float(m.attribute1),
                attribute2: convert_animated_float(m.attribute2),
                attribute3: convert_animated_float(m.attribute3),
                attribute4: convert_animated_float(m.attribute4),
                modifier: m.modifier,
            })
        }
        objects.push(SceneObject {
            position: [
                convert_animated_float(o.position[0].to_owned()),
                convert_animated_float(o.position[1].to_owned()),
                convert_animated_float(o.position[2].to_owned()),
            ],
            rotation: [
                convert_animated_float(o.rotation[0].to_owned()),
                convert_animated_float(o.rotation[1].to_owned()),
                convert_animated_float(o.rotation[2].to_owned()),
            ],
            scale: [
                convert_animated_float(o.scale[0].to_owned()),
                convert_animated_float(o.scale[1].to_owned()),
                convert_animated_float(o.scale[2].to_owned()),
            ],
            prim_type: o.prim_type,
            values: [
                convert_animated_float(o.values[0].to_owned()),
                convert_animated_float(o.values[1].to_owned()),
                convert_animated_float(o.values[2].to_owned()),
                convert_animated_float(o.values[3].to_owned()),
                convert_animated_float(o.values[4].to_owned()),
                convert_animated_float(o.values[5].to_owned()),
                convert_animated_float(o.values[6].to_owned()),
                convert_animated_float(o.values[7].to_owned()),
                convert_animated_float(o.values[8].to_owned()),
                convert_animated_float(o.values[9].to_owned()),
            ],
            // values: convert_script(),
            modifiers: mods,
        })
    }

    let scene = return Scene {
        objects: objects,
        group_modifiers: group_modifiers,
        camera: camera,
    };
}
