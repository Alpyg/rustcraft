use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use chunk::Chunk;
use iyes_progress::{Progress, ProgressReturningSystem};
use rendering::generation;

use crate::AppState;

pub mod chunk;
pub mod rendering;

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
                    generation::generate_flat_world,
                    generation::generate_world_meshes,
                    load_world.track_progress::<AppState>(),
                )
                    .chain(),
            )
            .insert_resource(World::new());
    }
}

fn load_world() -> Progress {
    true.into()
}
