use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_editor_pls::{
    controls::{self, EditorControls},
    EditorPlugin,
};
use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};

use block::BlockPlugin;
use fly_camera::FlyCameraPlugin;
use network::NetworkPlugin;
use player::PlayerPlugin;
use protocol::ProtocolPlugin;
use state::AppState;
use texture::TexturePlugin;
use world::WorldPlugin;

mod axis;
mod block;
mod core;
mod direction;
mod fly_camera;
mod network;
mod player;
mod prelude;
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
                    present_mode: PresentMode::AutoNoVsync,
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
    app.insert_resource(editor_controls());

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins((
        FlyCameraPlugin,
        TexturePlugin,
        BlockPlugin,
        NetworkPlugin,
        ProtocolPlugin,
        PlayerPlugin,
        WorldPlugin,
    ));

    app.insert_state(AppState::LoadingTextures);

    app.run();
}

fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}
