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
use world::WorldPlugin;

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
    LoadingWorld,
    MainMenu,
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
            .with_state_transition(AppState::LoadingModels, AppState::LoadingWorld)
            .with_state_transition(AppState::LoadingWorld, AppState::InGame),
        RapierPhysicsPlugin::<()>::default(),
    ));

    #[cfg(debug_assertions)]
    app.add_plugins((
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        RapierDebugRenderPlugin::default(),
    ));

    app.add_plugins((FlyCameraPlugin, TexturesPlugin, BlocksPlugin, WorldPlugin));

    app.add_systems(
        OnEnter(AppState::InGame),
        (
            //
            //spawn_model_test,
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
        PerspectiveProjection {
            fov: (90.0 / 360.0) * (std::f32::consts::PI * 2.0),
            ..default()
        },
        Transform::from_xyz(0.0, 3.0, 0.0).looking_at(Vec3::new(2.0, 2.0, 2.0), Vec3::Y),
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
