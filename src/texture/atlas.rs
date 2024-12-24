use std::marker::PhantomData;

use bevy::{
    asset::Handle, image::Image, math::URect, prelude::Resource, prelude::*, utils::HashMap,
};

use crate::block::Block;

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
