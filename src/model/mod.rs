use std::{collections::VecDeque, fs};

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;
use derive_more::{AsRef, Deref, DerefMut};
use serde_json::Value;

use self::block::{parse_block_model, BlockModel};

pub mod block;

#[derive(Reflect, Resource, InspectorOptions, Deref, DerefMut, AsRef, Debug, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct BlockModelRegistry(HashMap<String, BlockModel>);

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BlockModelRegistry>();
        app.add_systems(Startup, load_models);
    }
}

fn load_models(mut commands: Commands) {
    let mut blocks = HashMap::new();
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
            if !blocks.contains_key(parent.as_str().unwrap().split("/").last().unwrap()) {
                queue.push_back((ident.clone(), value.clone()));
                continue;
            }
        }

        let model = parse_block_model(&blocks, &value).unwrap();

        blocks.insert(ident, model);
    }

    commands.insert_resource(BlockModelRegistry(blocks))
}
