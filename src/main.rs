use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};

use block::{load_models, load_states, spawn_model_test, Block, BlockModelRegistry};
use fly_camera::{FlyCamera, FlyCameraPlugin};
//use network::NetworkPlugin;
//use player::PlayerPlugin;
//use protocol::ProtocolPlugin;
use texture::{build_texture_atlases, check_textures, load_textures, TextureAtlas};
//use world::WorldPlugin;

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
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        RapierPhysicsPlugin::<()>::default(),
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.insert_resource(TextureAtlas::<Block>::default());
    app.insert_resource(BlockModelRegistry::default());

    app.add_plugins((
        FlyCameraPlugin,
        //NetworkPlugin,
        //ProtocolPlugin,
        //PlayerPlugin,
        //WorldPlugin,
    ))
    .add_systems(OnEnter(AppState::LoadingTextures), load_textures)
    .add_systems(
        Update,
        (check_textures).run_if(in_state(AppState::LoadingTextures)),
    )
    .add_systems(
        OnEnter(AppState::LoadingModels),
        (
            build_texture_atlases,
            load_models,
            load_states,
            |mut commands: Commands| commands.set_state(AppState::InGame),
        )
            .chain(),
    )
    .add_systems(
        OnEnter(AppState::InGame),
        (spawn_model_test, spawn_camera).chain(),
    );

    app.insert_state(AppState::LoadingTextures);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        FlyCamera::default(),
        Camera3d::default(),
        Transform::from_xyz(0.5, 1.5, 7.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}
