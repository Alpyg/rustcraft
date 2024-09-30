use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{reflect::Reflect, utils::HashMap};
use bevy_mod_mesh_tools::{mesh_append, mesh_with_transform};
use serde::Deserialize;

use crate::texture::TextureRegistry;
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
    translation: Vec3,
    #[serde(default)]
    rotation: Vec3,
    #[serde(default)]
    scale: Vec3,
}

#[derive(Reflect, Deserialize, Debug, Default, Clone)]
pub struct ModelElement {
    #[serde(default)]
    from: Vec3,
    #[serde(default)]
    to: Vec3,
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
    origin: Vec3,
    #[serde(default)]
    axis: Axis,
    #[serde(default)]
    angle: f32,
    #[serde(default)]
    rescale: bool,
}

#[derive(Reflect, Deserialize, Debug, Default, Clone)]
pub struct ModelFace {
    #[serde(default)]
    uv: Vec4,
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
    models: &HashMap<String, BlockModel>,
    json: &serde_json::Value,
) -> BlockModel {
    let new_model: BlockModel = serde_json::from_value(json.clone()).unwrap();

    let parent = models.get(
        json.get("parent")
            .unwrap_or(&serde_json::Value::String("".to_string()))
            .as_str()
            .unwrap()
            .split("/")
            .last()
            .unwrap_or(""),
    );

    if parent.is_none() {
        return new_model;
    }

    let mut model = parent.unwrap().clone();

    model.ambient_occlusion = new_model.ambient_occlusion;
    for (key, val) in new_model.display {
        model.display.insert(key, val);
    }
    for (key, val) in new_model.textures {
        model.textures.insert(key, val);
    }
    for mut element in new_model.elements {
        let (from, to) = (element.from, element.to);
        for (direction, face) in element.faces.iter_mut() {
            if face.uv == Vec4::ZERO {
                face.uv = match direction {
                    Direction::Down => Vec4::new(from.x, from.z, to.x, to.z),
                    Direction::Up => Vec4::new(from.x, from.z, to.x, to.z),
                    Direction::North => Vec4::new(from.y, from.z, to.y, to.z),
                    Direction::South => Vec4::new(from.y, from.z, to.y, to.z),
                    Direction::West => Vec4::new(from.x, from.y, to.x, to.y),
                    Direction::East => Vec4::new(from.x, from.y, to.x, to.y),
                };
            }
        }
        model.elements.push(element);
    }

    model
}

pub fn build_block_mesh(model: &BlockModel, texture_registry: &Res<TextureRegistry>) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<[f32; 2]>::new())
    .with_inserted_indices(Indices::U32(Vec::<u32>::new()));

    for element in &model.elements {
        let mut transform = Transform::default();
        transform.rotate_around(
            element.rotation.origin,
            Quat::from_axis_angle(
                element.rotation.axis.into(),
                element.rotation.angle.to_radians(),
            ),
        );

        let element_mesh = create_element_mesh(element, &model.textures, texture_registry);
        let element_mesh = mesh_with_transform(&element_mesh, &transform).unwrap();

        mesh_append(&mut mesh, &element_mesh).unwrap();
    }

    mesh_with_transform(&mesh, &Transform::from_scale(Vec3::splat(1.0 / 16.0))).unwrap()
}

fn get_texture_uv(
    face_texture: &str,
    model_textures: &HashMap<String, String>,
    texture_registry: &Res<TextureRegistry>,
) -> URect {
    let texture_name_dbg = "debug".to_owned();
    let mut texture_name = model_textures
        .get(face_texture)
        .unwrap_or(&texture_name_dbg);
    while texture_name.starts_with("#") {
        match model_textures.get(&texture_name.clone().split_off(1)) {
            Some(texture) => {
                if texture == texture_name {
                    texture_name = &texture_name_dbg;
                    break;
                } else {
                    texture_name = texture;
                }
            }
            None => texture_name = &texture_name_dbg,
        }
    }

    let texture_name = texture_name.split("/").last().unwrap();
    let texture_id = &texture_registry
        .textures
        .get(&format!("minecraft:block/{}", texture_name))
        .unwrap_or(
            texture_registry
                .textures
                .get(&"minecraft:block/debug".to_string())
                .unwrap(),
        )
        .0;
    let texture_index = texture_registry
        .block_atlas
        .get_texture_index(texture_id)
        .unwrap();
    texture_registry.block_atlas.textures[texture_index]
}

fn create_element_mesh(
    el: &ModelElement,
    model_textures: &HashMap<String, String>,
    texture_registry: &Res<TextureRegistry>,
) -> Mesh {
    let (min, max) = (el.from, el.to);

    let mut vertices: Vec<[f32; 3]> = vec![];
    let mut indices: Vec<u32> = vec![];
    let mut uvs: Vec<[f32; 2]> = vec![];

    for (direction, face) in &el.faces {
        let face_texture = &face.texture.clone().split_off(1);
        let texture_uv = get_texture_uv(face_texture, model_textures, texture_registry);

        let v_len = vertices.len();
        let mut v;
        match direction {
            Direction::Up => {
                v = [
                    [min.x, max.y, min.z],
                    [min.x, max.y, max.z],
                    [max.x, max.y, max.z],
                    [max.x, max.y, min.z],
                ];
            }
            Direction::Down => {
                v = [
                    [min.x, min.y, max.z],
                    [min.x, min.y, min.z],
                    [max.x, min.y, min.z],
                    [max.x, min.y, max.z],
                ];
            }
            Direction::North => {
                v = [
                    [max.x, max.y, min.z],
                    [max.x, min.y, min.z],
                    [min.x, min.y, min.z],
                    [min.x, max.y, min.z],
                ];
            }
            Direction::South => {
                v = [
                    [min.x, max.y, max.z],
                    [min.x, min.y, max.z],
                    [max.x, min.y, max.z],
                    [max.x, max.y, max.z],
                ];
            }
            Direction::East => {
                v = [
                    [max.x, max.y, max.z],
                    [max.x, min.y, max.z],
                    [max.x, min.y, min.z],
                    [max.x, max.y, min.z],
                ];
            }
            Direction::West => {
                v = [
                    [min.x, max.y, min.z],
                    [min.x, min.y, min.z],
                    [min.x, min.y, max.z],
                    [min.x, max.y, max.z],
                ];
            }
        };
        indices.extend(&[0, 1, 2, 0, 2, 3].map(|i| i + v_len as u32));

        match face.rotation {
            90 => v.rotate_right(1),
            180 => v.rotate_left(2),
            270 => v.rotate_left(1),
            _ => {}
        };

        let padding = 0.1;
        let center = (face.uv.xy() + face.uv.zw()).div_euclid(Vec2::splat(2.0));
        let mut uv = [
            face.uv.xy() + (center - face.uv.xy()).signum() * padding,
            face.uv.xw() + (center - face.uv.xw()).signum() * padding,
            face.uv.zw() + (center - face.uv.zw()).signum() * padding,
            face.uv.zy() + (center - face.uv.zy()).signum() * padding,
        ];
        let uv: Vec<[f32; 2]> = uv
            .iter_mut()
            .map(|i: &mut Vec2| {
                [
                    (texture_uv.min.x as f32 + i.x) / 1024.0,
                    (texture_uv.min.y as f32 + i.y) / 1024.0,
                ]
            })
            .collect();

        vertices.extend(v);
        uvs.extend_from_slice(&uv);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}
