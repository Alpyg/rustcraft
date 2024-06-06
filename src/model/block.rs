#![allow(dead_code)]

use bevy::{reflect::Reflect, utils::HashMap};
use serde::Deserialize;
use serde_json::Value;

use crate::{axis::Axis, direction::Direction};

pub fn parse_block_model(
    _blocks: &HashMap<String, BlockModel>,
    json: &Value,
) -> anyhow::Result<BlockModel> {
    let model: BlockModel = serde_json::from_value(json.clone())?;

    // TODO: clone parent and override existing components

    Ok(model)
}

#[derive(Reflect, Deserialize, Debug, Default)]
pub struct BlockModel {
    #[serde(rename = "ambientocclusion", default)]
    ambient_occlusion: bool,
    #[serde(default)]
    display: Option<HashMap<String, ModelDisplay>>,
    #[serde(default)]
    textures: HashMap<String, String>,
    #[serde(default)]
    elements: Vec<ModelElement>,
}

#[derive(Reflect, Deserialize, Debug)]
pub struct ModelDisplay {
    #[serde(default)]
    translation: [f32; 3],
    #[serde(default)]
    rotation: [f32; 3],
    #[serde(default)]
    scale: [f32; 3],
}

#[derive(Reflect, Deserialize, Debug)]
pub struct ModelElement {
    #[serde(default)]
    from: [f32; 3],
    #[serde(default)]
    to: [f32; 3],
    #[serde(default)]
    rotation: Option<ModelRotation>,
    #[serde(default)]
    shade: bool,
    #[serde(default)]
    faces: HashMap<Direction, ModelFace>,
}

#[derive(Reflect, Deserialize, Debug)]
struct ModelRotation {
    #[serde(default)]
    origin: [f32; 3],
    axis: Axis,
    #[serde(default)]
    angle: f32,
    #[serde(default)]
    rescale: bool,
}

#[derive(Reflect, Deserialize, Debug)]
pub struct ModelFace {
    #[serde(default)]
    uv: Option<[f32; 4]>,
    #[serde(default)]
    texture: String,
    #[serde(default)]
    cullface: Option<CullfaceDirection>,
    #[serde(default)]
    rotation: Option<i32>,
    #[serde(rename = "tintindex", default)]
    tint_index: Option<i32>,
}

#[derive(Reflect, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CullfaceDirection {
    Bottom,
    Down,
    Up,
    North,
    South,
    West,
    East,
}
