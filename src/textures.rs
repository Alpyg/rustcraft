use bevy::{ecs::system::Command, prelude::*, utils::HashMap};
use indexmap::IndexSet;

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_texture_atlas.run_if(resource_exist::<TextureAtlasBuilder>())));
        app.init_resource::<TextureHandles>();
    }
}

#[derive(Resource)]
struct TextureAtlasBuilder(IndexSet<Handle<Image>>);

pub struct MakeTextureAtlas(Vec<BlockType>);
impl MakeTextureAtlas {
    pub fn new(blocks: impl Iterator<Item = BlockType>) -> Self {
        MakeTextureAtlas(blocks.collect())
    }
}

impl Command for MakeTextureAtlas {
    fn apply(self, world: &mut World) {
        let mut need_textures = IndexSet::with_capacity(self.0.len);
        let mut map = HashMap::with_capacity(self.0.len);
        let asset_server = world.resource::<AssetServer>();
        for block in self.0.iter() {
            for path in block.get_texture_paths() {
                let handle = asset_server.load::<>(*path);
                let temp = handle.clone_weak();
            }
        }
    }
}
