use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};

use network::NetworkPlugin;
use player::PlayerPlugin;
use protocol::ProtocolPlugin;
use states::AppState;

mod core;
mod network;
mod player;
mod prelude;
mod states;

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
        EditorPlugin::default(),
        RapierPhysicsPlugin::<()>::default(),
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins((NetworkPlugin, ProtocolPlugin, PlayerPlugin));

    app.insert_state(AppState::InGame);

    app.run();
}
