use std::{collections::VecDeque, fs};

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;

use crate::{
    block::blockstate::BlockStateMultipartWhen, fly_camera::FlyCamera, state::AppState,
    texture::TextureRegistry,
};

use self::{
    blockstate::{BlockDefinition, BlockState, BlockStateModel},
    model::{build_block_mesh, parse_block_model, BlockModel},
};

pub mod blockstate;
pub mod model;

#[derive(Reflect, Resource, InspectorOptions, Debug, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct BlockModelRegistry {
    pub models: HashMap<String, BlockModel>,
    pub meshes: HashMap<String, Handle<Mesh>>,
}

#[derive(Resource, Debug, Default)]
pub struct BlockStateRegistry {
    pub block_definitions: HashMap<String, BlockDefinition>,
    pub blockstate_models: HashMap<i32, Vec<BlockStateModel>>,
}

pub struct BlockPlugin;
impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BlockModelRegistry>();
        app.add_systems(OnEnter(AppState::LoadingModels), load_models);
        app.add_systems(
            OnEnter(AppState::LoadingModels),
            load_states.after(load_models),
        );
        app.add_systems(OnEnter(AppState::LoadingModels), spawn.after(load_states));
    }
}

fn load_models(
    mut commands: Commands,
    mut meshes_res: ResMut<Assets<Mesh>>,
    texture_registry: Res<TextureRegistry>,
) {
    let mut models = HashMap::new();
    let mut meshes = HashMap::new();
    let blocks_path = "assets/assets/minecraft/models/block";

    let paths = fs::read_dir(blocks_path).unwrap();

    let mut queue: VecDeque<(String, serde_json::Value)> = VecDeque::new();

    for path in paths {
        let path = path.unwrap().path();
        let file_path = path.to_str().unwrap();
        let json_str = fs::read_to_string(file_path).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        queue.push_back((
            path.file_stem().unwrap().to_str().unwrap().to_owned(),
            json_value,
        ));
    }

    while !queue.is_empty() {
        let (ident, value) = queue.pop_front().unwrap();
        if let Some(parent) = value.get("parent") {
            if !models.contains_key(parent.as_str().unwrap().split("/").last().unwrap()) {
                queue.push_back((ident.clone(), value.clone()));
                continue;
            }
        }

        let model = parse_block_model(&models, &value);
        let mesh = build_block_mesh(&model, &texture_registry);
        let mesh_handle = meshes_res.add(mesh);

        models.insert(ident.clone(), model.clone());
        meshes.insert(ident.clone(), mesh_handle);
    }

    commands.insert_resource(BlockModelRegistry { models, meshes });
}

fn load_states(mut commands: Commands) {
    let data = fs::read_to_string("assets/reports/blocks.json").unwrap();
    let value: serde_json::Value = serde_json::from_str(&data).unwrap();

    let block_definitions: HashMap<String, BlockDefinition> =
        serde_json::from_value(value).unwrap();

    // TODO: Load individual blockstate
    //
    // TODO: Generate meshes and keep them seperate depending on cullface
    // maybe put non cullable faces in one mesh and the cullable ones in a HashMap
    // take neede cullable faces and merge them with the main faces and generate a final mesh on
    // demand
    let mut blockstate_models = HashMap::new();
    for (block, blockstate_definition) in block_definitions.iter() {
        let data = fs::read_to_string(format!(
            "assets/assets/minecraft/blockstates/{}.json",
            block.clone().split_off("minecraft:".len())
        ))
        .unwrap();

        for (id, state) in &blockstate_definition.states {
            let mut models = vec![];
            match serde_json::from_str(&data).unwrap() {
                BlockState::Variants(variants) => {
                    'variants: for (variant_key, variant) in &variants {
                        if variant_key.is_empty() {
                            blockstate_models.insert(*id, variant.0.clone());
                            break 'variants;
                        }
                        let variant_properties: HashMap<&str, &str> = variant_key
                            .split(',')
                            .map(|pair| {
                                let mut iter = pair.split('=');
                                (iter.next().unwrap(), iter.next().unwrap())
                            })
                            .collect();

                        if variant_properties.iter().all(|(key, value)| {
                            state.properties.get(*key).map_or(false, |v| v == value)
                        }) {
                            models = variant.0.clone();
                            break 'variants;
                        }
                    }
                }
                BlockState::Multipart(multipart) => {
                    for part in multipart {
                        match part.when {
                            Some(when) => {
                                if match when {
                                    BlockStateMultipartWhen::State(conditions) => conditions
                                        .iter()
                                        .all(|(key, value)| match state.properties.get(key) {
                                            Some(state_value) => {
                                                value.split('|').any(|v| v == state_value)
                                            }
                                            None => false,
                                        }),
                                    BlockStateMultipartWhen::Or(conditions) => {
                                        conditions.iter().any(|conditions| {
                                            conditions.iter().all(|(key, value)| {
                                                match state.properties.get(key) {
                                                    Some(state_value) => {
                                                        value.split('|').any(|v| v == state_value)
                                                    }
                                                    None => false,
                                                }
                                            })
                                        })
                                    }

                                    BlockStateMultipartWhen::And(conditions) => {
                                        conditions.iter().all(|conditions| {
                                            conditions.iter().all(|(key, value)| {
                                                match state.properties.get(key) {
                                                    Some(state_value) => {
                                                        value.split('|').any(|v| v == state_value)
                                                    }
                                                    None => false,
                                                }
                                            })
                                        })
                                    }
                                } {
                                    models.extend(part.apply);
                                }
                            }
                            None => models.extend(part.apply),
                        }
                    }
                }
            }
            blockstate_models.insert(*id, models);
        }
    }

    commands.insert_resource(BlockStateRegistry {
        block_definitions,
        blockstate_models,
    })
}

fn spawn(
    mut commands: Commands,
    block_models: Res<BlockModelRegistry>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    texture_registry: Res<TextureRegistry>,
) {
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        mesh: block_models.meshes.get("acacia_stairs").unwrap().clone(),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_registry.block.clone()),
            alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            ..default()
        }),
        ..default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-1.0, 0.0, 0.0),
        mesh: block_models.meshes.get("piston_head").unwrap().clone(),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_registry.block.clone()),
            alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            ..default()
        }),
        ..default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(1.0, 2.0, 0.0),
        mesh: block_models.meshes.get("lectern").unwrap().clone(),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_registry.block.clone()),
            alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            ..default()
        }),
        ..default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-1.0, 2.0, 0.0),
        mesh: block_models.meshes.get("beehive").unwrap().clone(),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_registry.block.clone()),
            alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            ..default()
        }),
        ..default()
    });

    let camera_and_light_transform =
        Transform::from_xyz(0.5, 1.5, 7.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);

    commands.spawn((
        FlyCamera::default(),
        Camera3dBundle {
            transform: camera_and_light_transform,
            ..default()
        },
    ));
}
