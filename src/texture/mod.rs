use anyhow::Result;
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::ConfigureLoadingState, LoadingState, LoadingStateAppExt, LoadingStateSet,
    },
};
use iyes_progress::{Progress, ProgressReturningSystem};

use crate::{block::Block, AppState};

mod atlas;

pub use atlas::TextureAtlas;

#[derive(AssetCollection, Resource)]
struct TextureCollection {
    #[asset(path = "assets/minecraft/textures/block", collection(typed))]
    blocks: Vec<Handle<Image>>,
}

pub struct TexturesPlugin;
impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingTextures).load_collection::<TextureCollection>(),
        )
        .add_systems(OnExit(AppState::LoadingTextures), build_texture_atlases)
        .add_systems(
            Update,
            (load_progress.track_progress::<AppState>())
                .chain()
                .run_if(in_state(AppState::LoadingTextures))
                .after(LoadingStateSet(AppState::LoadingTextures)),
        )
        .insert_resource(TextureAtlas::<Block>::default());
    }
}

fn build_texture_atlases(
    asset_server: Res<AssetServer>,
    texture_collection: Res<TextureCollection>,
    mut block_atlas: ResMut<TextureAtlas<Block>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut images: ResMut<Assets<Image>>,
) {
    if is_recursively_loaded(&texture_collection.blocks, &asset_server) {
        if let Ok((image, uvs)) =
            build_texture_atlas_from_dir(&texture_collection.blocks, &mut layouts, &mut images)
        {
            block_atlas.texture = image;
            block_atlas.uvs = uvs;
        }
    }
}

fn build_texture_atlas_from_dir(
    block_textures: &Vec<Handle<Image>>,
    layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    images: &mut ResMut<Assets<Image>>,
) -> Result<(Handle<Image>, HashMap<String, URect>)> {
    let mut atlas_builder = TextureAtlasBuilder::default();

    let mut texture_map: HashMap<AssetId<Image>, String> = HashMap::new();
    for handle in block_textures.iter() {
        if let Some(texture) = images.get(handle.id()) {
            atlas_builder.add_texture(Some(handle.id()), texture);

            texture_map.insert(
                handle.id(),
                format!(
                    "minecraft:block/{}",
                    handle
                        .path()
                        .unwrap()
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                ),
            );
        }
    }

    match atlas_builder.build() {
        Ok((layout, sources, image)) => {
            let mut uvs: HashMap<String, URect> = HashMap::new();
            for (asset_id, texture_name) in texture_map {
                if let Some(&index) = sources.texture_ids.get(&asset_id) {
                    if let Some(uv) = layout.textures.get(index) {
                        uvs.insert(texture_name, uv.clone());
                    }
                }
            }

            layouts.add(layout);
            let image = images.add(image);

            Ok((image, uvs))
        }
        Err(error) => Err(error.into()),
    }
}

fn load_progress(
    asset_server: Res<AssetServer>,
    texture_collection: Res<TextureCollection>,
) -> Progress {
    if is_recursively_loaded(&texture_collection.blocks, &asset_server) {
        true.into()
    } else {
        false.into()
    }
}

fn is_recursively_loaded(handles: &Vec<Handle<Image>>, asset_server: &AssetServer) -> bool {
    handles.iter().all(|handle| {
        asset_server
            .get_recursive_dependency_load_state(&*handle)
            .map(|state| state.is_loaded())
            .unwrap_or(false)
    })
}
