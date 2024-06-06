use anyhow::Ok;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{reflect::Reflect, utils::HashMap};
use bevy_mod_mesh_tools::{mesh_append, mesh_with_transform};
use serde::Deserialize;
use serde_json::Value;

use crate::{axis::Axis, direction::Direction};

#[derive(Reflect, Deserialize, Debug, Default, Clone)]
pub struct BlockModel {
    #[serde(rename = "ambientocclusion", default)]
    ambient_occlusion: bool,
    #[serde(default)]
    display: HashMap<String, ModelDisplay>,
    #[serde(default)]
    textures: HashMap<String, String>,
    #[serde(default)]
    elements: Vec<ModelElement>,
}

#[derive(Reflect, Deserialize, Debug, Default, Clone)]
pub struct ModelDisplay {
    #[serde(default)]
    translation: [f32; 3],
    #[serde(default)]
    rotation: [f32; 3],
    #[serde(default)]
    scale: [f32; 3],
}

#[derive(Reflect, Deserialize, Debug, Default, Clone)]
pub struct ModelElement {
    #[serde(default)]
    from: [f32; 3],
    #[serde(default)]
    to: [f32; 3],
    #[serde(default)]
    rotation: ModelRotation,
    #[serde(default)]
    shade: bool,
    #[serde(default)]
    faces: HashMap<Direction, ModelFace>,
}

#[derive(Reflect, Deserialize, Debug, Default, Clone)]
struct ModelRotation {
    #[serde(default)]
    origin: [f32; 3],
    axis: Axis,
    #[serde(default)]
    angle: f32,
    #[serde(default)]
    rescale: bool,
}

#[derive(Reflect, Deserialize, Debug, Default, Clone)]
pub struct ModelFace {
    #[serde(default)]
    uv: [f32; 4],
    #[serde(default)]
    texture: String,
    #[serde(default)]
    cullface: CullfaceDirection,
    #[serde(default)]
    rotation: i32,
    #[serde(rename = "tintindex", default)]
    tint_index: i32,
}

#[derive(Reflect, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CullfaceDirection {
    #[default]
    Bottom,
    Down,
    Up,
    North,
    South,
    West,
    East,
}

pub fn parse_block_model(
    blocks: &HashMap<String, (BlockModel, Handle<Mesh>)>,
    json: &Value,
) -> anyhow::Result<BlockModel> {
    let new_model: BlockModel = serde_json::from_value(json.clone())?;

    let parent = blocks.get(
        json.get("parent")
            .unwrap_or(&Value::String("".to_string()))
            .as_str()
            .unwrap()
            .split("/")
            .last()
            .unwrap_or(""),
    );

    if parent.is_none() {
        return Ok(new_model);
    }

    let mut model = parent.unwrap().clone().0;

    model.ambient_occlusion = new_model.ambient_occlusion;
    for (key, val) in new_model.display {
        model.display.insert(key, val);
    }
    for (key, val) in new_model.textures {
        model.textures.insert(key, val);
    }
    for element in new_model.elements {
        model.elements.push(element);
    }

    Ok(model)
}

pub fn build_block_mesh(model: &BlockModel) -> anyhow::Result<Mesh> {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<[f32; 2]>::new())
    .with_inserted_indices(Indices::U32(Vec::<u32>::new()));

    for element in &model.elements {
        let mut transform = Transform::from_translation(
            (Into::<Vec3>::into(element.from) + Into::<Vec3>::into(element.to)) / 2.0,
        );
        transform.rotate_around(
            Into::<Vec3>::into(element.rotation.origin),
            Quat::from_axis_angle(
                element.rotation.axis.into(),
                element.rotation.angle.to_radians(),
            ),
        );

        let element_mesh = mesh_with_transform(
            &Cuboid::from_corners(element.from.into(), element.to.into()).mesh(),
            &transform,
        )
        .unwrap();

        mesh_append(&mut mesh, &element_mesh).unwrap();
    }

    let mesh = mesh_with_transform(&mesh, &Transform::from_scale(Vec3::splat(1.0 / 16.0))).unwrap();

    Ok(mesh)
}
