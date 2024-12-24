use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};

use block::{Block, BlockStateRegistry, BlocksPlugin};
use fly_camera::{FlyCamera, FlyCameraPlugin};
use iyes_progress::ProgressPlugin;
use texture::{TextureAtlas, TexturesPlugin};

mod axis;
mod block;
mod core;
mod direction;
mod fly_camera;
mod network;
mod player;
mod prelude;
mod texture;
mod world;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    LoadingTextures,
    LoadingModels,
    #[allow(dead_code)]
    MainMenu,
    #[allow(dead_code)]
    InGame,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rustcraft".to_owned(),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        ProgressPlugin::<AppState>::new()
            .with_state_transition(AppState::LoadingTextures, AppState::LoadingModels)
            .with_state_transition(AppState::LoadingModels, AppState::InGame),
        RapierPhysicsPlugin::<()>::default(),
    ));

    #[cfg(debug_assertions)]
    app.add_plugins((
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        RapierDebugRenderPlugin::default(),
    ));

    app.add_plugins((FlyCameraPlugin, TexturesPlugin, BlocksPlugin));

    app.add_systems(
        OnEnter(AppState::InGame),
        (
            //debug_world,
            spawn_model_test,
            spawn_camera,
        )
            .chain(),
    );

    app.insert_state(AppState::LoadingTextures);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        FlyCamera::default(),
        Camera3d::default(),
        Transform::from_xyz(0.5, 1.5, 7.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        Msaa::Off,
    ));
}

pub fn spawn_model_test(
    mut commands: Commands,
    blockstates: Res<BlockStateRegistry>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    atlas: Res<TextureAtlas<Block>>,
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh3d(blockstates.blockstates_meshes.get(&7919).unwrap().clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(atlas.texture.clone()),
            alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            ..default()
        })),
    ));
}

pub fn debug_world(
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
