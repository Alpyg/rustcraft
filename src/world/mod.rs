use std::ops::Deref;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    utils::HashMap,
};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_mod_mesh_tools::{mesh_append, mesh_with_transform};
use chunk::Chunk;
use iyes_progress::{Progress, ProgressReturningSystem};

use crate::{
    block::{Block, BlockStateRegistry},
    texture::TextureAtlas,
    AppState,
};

mod chunk;

#[derive(Resource, Debug)]
pub struct World {
    chunks: HashMap<IVec2, Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(LoadingState::new(AppState::LoadingWorld))
            .add_systems(
                OnEnter(AppState::LoadingWorld),
                (
                    generate_flat_world,
                    generate_world_meshes,
                    load_world.track_progress::<AppState>(),
                )
                    .chain(),
            )
            .insert_resource(World::new());
    }
}

fn generate_world_meshes(
    mut commands: Commands,
    world: ResMut<World>,
    blockstates: Res<BlockStateRegistry>,
    atlas: Res<TextureAtlas<Block>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (chunk_pos, chunk) in world.chunks.iter() {
        let mut chunk_mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new())
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<[f32; 2]>::new())
        .with_inserted_indices(Indices::U32(Vec::<u32>::new()));

        for (block_pos, block_id) in chunk.iter() {
            let block_mesh = meshes
                .get(
                    blockstates
                        .blockstates_meshes
                        .get(block_id.deref())
                        .unwrap(),
                )
                .unwrap();
            let transform = Transform::from_translation(block_pos.as_vec3());
            let world_pos_block_mesh = mesh_with_transform(&block_mesh, &transform).unwrap();

            mesh_append(&mut chunk_mesh, &world_pos_block_mesh).unwrap();
        }
        mesh_with_transform(&chunk_mesh, &Transform::from_scale(Vec3::splat(1.0 / 16.0))).unwrap();
        let chunk_mesh = meshes.add(chunk_mesh);

        let chunk_pos = Vec3::new(chunk_pos.x as f32, 0.0, chunk_pos.y as f32);
        commands.spawn((
            Transform::from_translation(chunk_pos),
            Mesh3d(chunk_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(atlas.texture.clone()),
                alpha_mode: AlphaMode::Mask(0.0),
                unlit: true,
                ..default()
            })),
        ));
    }
}

fn generate_flat_world(mut world: ResMut<World>) {
    let chunk = generate_flat_chunk();
    world.chunks.insert(IVec2::new(0, 0), chunk);
}

fn generate_flat_chunk() -> Chunk {
    let bedrock_id = 79;
    let dirt_id = 10;
    let grass_id = 9;

    let mut chunk = Chunk::new();
    for x in 0..16 {
        for z in 0..16 {
            chunk.insert(IVec3::new(x, 0, z), bedrock_id.into());
        }
    }
    for x in 0..16 {
        for z in 0..16 {
            for y in 1..3 {
                chunk.insert(IVec3::new(x, y, z), dirt_id.into());
            }
        }
    }
    for x in 0..16 {
        for z in 0..16 {
            chunk.insert(IVec3::new(x, 3, z), grass_id.into());
        }
    }
    chunk
}

fn generate_debug_world(
    mut commands: Commands,
    blockstates: Res<BlockStateRegistry>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    atlas: Res<TextureAtlas<Block>>,
) {
    for (state_id, state_mesh_handle) in &blockstates.blockstates_meshes {
        let (x, z) = (state_id / 164, state_id % 164);
        commands.spawn((
            Transform::from_xyz(1.0 + 2.0 * x as f32, 0.0, 1.0 + 2.0 * z as f32),
            Mesh3d(state_mesh_handle.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(atlas.texture.clone()),
                alpha_mode: AlphaMode::Mask(0.0),
                unlit: true,
                ..default()
            })),
        ));
    }
}

fn load_world() -> Progress {
    true.into()
}
