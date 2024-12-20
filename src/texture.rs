use std::marker::PhantomData;

use anyhow::Result;
use bevy::{asset::LoadedFolder, prelude::*, utils::HashMap};

use crate::{block::Block, AppState};

#[derive(Resource, Debug, Default)]
pub struct BlockTextureFolder(Handle<LoadedFolder>);

#[derive(Resource, Debug, Default, Clone)]
pub struct TextureAtlas<T> {
    pub texture: Handle<Image>,
    pub uvs: HashMap<String, URect>,
    _d: PhantomData<T>,
}

impl<T> TextureAtlas<T> {
    pub fn get_texture_uv(
        &self,
        face_texture: &str,
        model_textures: &HashMap<String, String>,
        atlas: &Res<TextureAtlas<Block>>,
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
        *atlas
            .uvs
            .get(&format!("minecraft:block/{}", texture_name))
            .unwrap_or(atlas.uvs.get(&"minecraft:block/debug".to_owned()).unwrap())
    }
}

pub fn load_textures(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(BlockTextureFolder(
        server.load_folder("assets/minecraft/textures/block"),
    ));
}

pub fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    block_texture_folder: Res<BlockTextureFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&block_texture_folder.0) {
            next_state.set(AppState::LoadingModels);
        }
    }
}
pub fn build_texture_atlases(
    block_texture_handles: Res<BlockTextureFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut block_atlas: ResMut<TextureAtlas<Block>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut images: ResMut<Assets<Image>>,
) {
    let block_texture_folder = loaded_folders.get(&block_texture_handles.0).unwrap();

    if let Ok((image, uvs)) =
        build_texture_atlas_from_dir(block_texture_folder, &mut layouts, &mut images)
    {
        block_atlas.texture = image;
        block_atlas.uvs = uvs;

        println!("texture: {:?}", block_atlas.texture);
        println!("uvs: {:?}", block_atlas.uvs.len());
    }
}

fn build_texture_atlas_from_dir(
    loaded_folder: &LoadedFolder,
    layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    images: &mut ResMut<Assets<Image>>,
) -> Result<(Handle<Image>, HashMap<String, URect>)> {
    let mut atlas_builder = TextureAtlasBuilder::default();

    let mut texture_map: HashMap<AssetId<Image>, String> = HashMap::new();
    for handle in loaded_folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        if let Some(texture) = images.get(id) {
            atlas_builder.add_texture(Some(id), texture);

            texture_map.insert(
                id,
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
