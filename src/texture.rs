use bevy::{asset::LoadedFolder, prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;

use crate::state::AppState;

#[derive(Reflect, Resource, InspectorOptions, Debug, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct TextureFolder(Handle<LoadedFolder>);

#[derive(Resource, Debug)]
pub struct TextureRegistry {
    pub block: Handle<Image>,
    pub block_atlas: TextureAtlasLayout,
    pub textures: HashMap<String, (Handle<Image>, AssetId<Image>)>,
}

pub struct TexturePlugin;
impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadingTextures), load_textures_folder);
        app.add_systems(
            Update,
            check_textures.run_if(in_state(AppState::LoadingTextures)),
        );
        app.add_systems(OnEnter(AppState::ProcessingTextures), create_texture_atlas);
    }
}

fn load_textures_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TextureFolder(
        asset_server.load_folder("assets/minecraft/textures/block"),
    ));
}

fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    texture_folder: Res<TextureFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&texture_folder.0) {
            next_state.set(AppState::ProcessingTextures);
        }
    }
}

fn create_texture_atlas(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    texture_folder: Res<TextureFolder>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let loaded_folder = loaded_folders.get(&texture_folder.0).unwrap();

    let mut textures_map = HashMap::new();
    for handle in loaded_folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        if let Some(texture) = textures.get(id) {
            texture_atlas_builder.add_texture(Some(id), texture);
            if textures.get(id).is_some() {
                let texture_handle = handle.clone().typed_unchecked::<Image>();
                let file_name = handle.path().unwrap().path().file_stem().unwrap();
                textures_map.insert(
                    format!("minecraft:block/{}", file_name.to_str().unwrap()),
                    (texture_handle.clone(), id),
                );
            };
        }
    }

    let (layout, texture) = texture_atlas_builder.build().unwrap();
    let texture_handle = textures.add(texture);
    texture_atlases.add(layout.clone());

    commands.insert_resource(TextureRegistry {
        block: texture_handle,
        block_atlas: layout,
        textures: textures_map,
    });

    next_state.set(AppState::LoadingModels);
}
