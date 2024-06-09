use std::{collections::VecDeque, fs};

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;
use serde_json::Value;

use crate::{fly_camera::FlyCamera, state::AppState, texture::TextureRegistry};

use self::block::{build_block_mesh, parse_block_model, BlockModel};

pub mod block;

#[derive(Reflect, Resource, InspectorOptions, Debug, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct BlockModelRegistry {
    pub models: HashMap<String, BlockModel>,
    pub meshes: HashMap<String, Handle<Mesh>>,
}

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BlockModelRegistry>();
        app.add_systems(OnEnter(AppState::LoadingModels), load_models);
        app.add_systems(OnEnter(AppState::LoadingModels), spawn.after(load_models));
    }
}

fn load_models(
    mut commands: Commands,
    mut meshes_res: ResMut<Assets<Mesh>>,
    texture_registry: Res<TextureRegistry>,
) {
    let mut models = HashMap::new();
    let mut meshes = HashMap::new();
    let blocks_path = "assets/1.20.4/assets/minecraft/models/block";

    let paths = fs::read_dir(blocks_path).unwrap();

    let mut queue: VecDeque<(String, Value)> = VecDeque::new();

    for path in paths {
        let path = path.unwrap().path();
        let file_path = path.to_str().unwrap();
        let json_str = fs::read_to_string(file_path).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

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

fn spawn(
    mut commands: Commands,
    block_models: Res<BlockModelRegistry>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    texture_registry: Res<TextureRegistry>,
) {
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        mesh: block_models.meshes.get("lectern").unwrap().clone(),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_registry.block.clone()),
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
            unlit: true,
            ..default()
        }),
        ..default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(1.0, 2.0, 0.0),
        mesh: block_models.meshes.get("stone").unwrap().clone(),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_registry.block.clone()),
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
            unlit: true,
            ..default()
        }),
        ..default()
    });

    let camera_and_light_transform =
        Transform::from_xyz(-3.0, 1.7, -3.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);

    commands.spawn((
        FlyCamera::default(),
        Camera3dBundle {
            transform: camera_and_light_transform,
            ..default()
        },
    ));
}
