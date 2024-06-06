use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};

use model::ModelPlugin;
use network::NetworkPlugin;
use player::PlayerPlugin;
use protocol::ProtocolPlugin;
use registry::RegistryPlugin;
use state::AppState;
use texture::TexturePlugin;
use world::WorldPlugin;

mod axis;
mod core;
mod direction;
mod model;
mod network;
mod player;
mod prelude;
mod registry;
mod state;
mod texture;
mod world;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rustcraft".to_owned(),
                    resolution: (720., 480.).into(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        EditorPlugin::default(),
        RapierPhysicsPlugin::<()>::default(),
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins((
        TexturePlugin,
        ModelPlugin,
        RegistryPlugin,
        NetworkPlugin,
        ProtocolPlugin,
        PlayerPlugin,
        WorldPlugin,
    ));

    app.insert_state(AppState::Loading);

    app.run();
}
