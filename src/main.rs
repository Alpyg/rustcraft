use avian3d::prelude::*;
use bevy::{prelude::*, window::PresentMode};
use bevy_enhanced_input::EnhancedInputPlugin;

use block::{Block, BlockStateRegistry, BlocksPlugin};
use editor::EditorPlugin;
use fly_camera::{FlyCamera, FlyCameraPlugin};
use iyes_progress::ProgressPlugin;
use player::PlayerPlugin;
use texture::{TextureAtlas, TexturesPlugin};
use world::WorldPlugin;

mod axis;
mod block;
mod core;
mod direction;
mod editor;
mod fly_camera;
mod network;
mod player;
mod prelude;
mod texture;
mod world;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    LoadingTextures,
    LoadingModels,
    LoadingWorld,
    MainMenu,
    InGame,
    Pause,
}

#[derive(PhysicsLayer, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum GameLayer {
    #[default]
    Default,
    Player,
    World,
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
        PhysicsPlugins::default(),
        EnhancedInputPlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(EditorPlugin);

    app.add_plugins((
        TexturesPlugin,
        BlocksPlugin,
        WorldPlugin,
        PlayerPlugin,
        //FlyCameraPlugin,
        //
    ));

    app.add_systems(
        OnEnter(AppState::InGame),
        (
            spawn_model_test,
            //spawn_camera,
            //
        )
            .chain(),
    );

    app.init_state::<AppState>();

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
        Transform::from_xyz(8.0, 4.0, 4.0),
        Mesh3d(blockstates.meshes.get(&7919).unwrap().clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(atlas.texture.clone()),
            alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            ..default()
        })),
    ));
}
