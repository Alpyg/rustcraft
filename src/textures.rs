use std::fs;

use bevy::{asset::LoadedFolder, prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;

use crate::states::AppState;

#[derive(Reflect, Resource, InspectorOptions, Debug, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct TextureFolder(Handle<LoadedFolder>);

#[derive(Reflect, Resource, InspectorOptions, Debug, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct Textures {
    pub block: Handle<Image>,
    pub textures: HashMap<String, Handle<Image>>,
}

pub struct TexturePlugin;
impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Textures>();
        app.add_systems(OnEnter(AppState::Loading), load_textures_folder);
        app.add_systems(Update, check_textures.run_if(in_state(AppState::Loading)));
        app.add_systems(OnEnter(AppState::Processing), create_texture_atlas);
    }
}

fn load_textures_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TextureFolder(
        asset_server.load_folder("1.20.4/assets/minecraft/textures/block"),
    ));
}

fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    texture_folder: Res<TextureFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&texture_folder.0) {
            next_state.set(AppState::Processing);
        }
    }
}

fn create_texture_atlas(
    mut commands: Commands,
    texture_folder: Res<TextureFolder>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let loaded_folder = loaded_folders.get(&texture_folder.0).unwrap();

    for handle in loaded_folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        if let Some(texture) = textures.get(id) {
            texture_atlas_builder.add_texture(Some(id), texture);
        };
    }

    let (layout, texture) = texture_atlas_builder.finish().unwrap();
    let texture = textures.add(texture);

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: texture.clone(),
            transform: Transform {
                scale: Vec3::splat(0.99),
                ..default()
            },
            ..default()
        },
        Name::new("Atlas"),
    ));

    commands.insert_resource(Textures {
        block: texture,
        textures: HashMap::new(),
    });
}

fn load_textures_old(
    server: &Res<AssetServer>,
    namespace: &str,
    path: &str,
) -> HashMap<String, Handle<Image>> {
    let path = format!("assets/1.20.4/assets/minecraft/textures/{}", path);
    let files = fs::read_dir(&path).unwrap();

    let mut textures = HashMap::new();
    for file in files {
        let file = file.unwrap().path();
        if file.extension().unwrap().to_str().unwrap() != "png" {
            continue;
        }
        let filepath = file.strip_prefix("assets").unwrap();
        let filename = file.file_stem().unwrap();

        let handle: Handle<Image> = server.load(filepath.to_owned());
        let key = format!("{}/{}", namespace, filename.to_str().unwrap()).to_owned();
        textures.insert(key, handle);
    }

    textures
}
