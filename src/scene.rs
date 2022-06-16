use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde()]
pub struct SceneModifier{
    pub attribute0: f64,
    pub attribute1: f64,
    pub attribute2: f64,
    pub attribute3: f64,
    pub attribute4: f64,
    pub modifier: i32,
}

#[derive(Debug, Deserialize)]
#[serde()]
pub struct SceneGroupModifier{
    pub prim0: i32,
    pub prim1: i32,
    pub prim2: i32,
    pub prim3: i32,
    pub prim_attribute: f64,
    pub modifier: i32,
}

#[derive(Debug, Deserialize)]
#[serde()]
pub struct SceneObject{
    pub position: [f64;3],
    pub rotation: [f64;3],
    pub scale: [f64;3],
    pub prim_type: i32,
    pub values: [f64;10],
    pub modifiers: std::vec::Vec<SceneModifier>
}

#[derive(Debug, Deserialize)]
#[serde()]
pub struct Scene{
    pub objects: std::vec::Vec<SceneObject>,
    pub group_modifiers: std::vec::Vec<SceneGroupModifier>
}

pub fn load_scene(path: &str) -> Scene{
    let contents = fs::read_to_string(path)
    .expect("Something went wrong reading the file");
    let json: Scene =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");
    return json;
}