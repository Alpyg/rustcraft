use avian3d::prelude::*;
use bevy::prelude::*;
use controls::{survival::SurvivalControls, Acceleration, Velocity};
use protocol::{
    packets::{
        ClientKeepAlivePlay, ConfirmTeleport, ServerKeepAlivePlay, SynchronizePlayerPosition,
    },
    PacketEncoder, PacketEvent,
};

use crate::GameLayer;

mod controls;

const PLAYER_HEIGHT: f32 = 1.8;
const PLAYER_EYE_HEIGHT: f32 = 1.62;
const PLAYER_WIDTH: f32 = 0.6;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCamera;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            controls::plugin,
            //
        ))
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (handle_keep_alive, handle_syncrhonize_player_position),
        );
    }
}

fn spawn_player(mut commands: Commands) {
    let player_transform = Transform::from_xyz(8.0, 8.0, 8.0);
    commands
        .spawn((
            Player,
            Name::new("Player"),
            player_transform,
            Visibility::default(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Acceleration::default(),
            Velocity::default(),
            SurvivalControls,
        ))
        .with_children(|p| {
            p.spawn((
                PlayerCamera,
                Name::new("Player Camera"),
                player_transform.with_translation(Vec3::new(0.0, PLAYER_EYE_HEIGHT, 0.0)),
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ));
            p.spawn((
                Name::new("Player Collider"),
                player_transform.with_translation(Vec3::new(0.0, PLAYER_HEIGHT / 2.0, 0.0)),
                Collider::cuboid(PLAYER_WIDTH, PLAYER_HEIGHT, PLAYER_WIDTH),
                CollisionLayers::new(GameLayer::Player, [GameLayer::World]),
            ));
        });
}

fn handle_keep_alive(mut encoder: ResMut<PacketEncoder>, mut pkts: EventReader<PacketEvent>) {
    for pkt in pkts.read() {
        if let Some(pkt) = pkt.decode::<ClientKeepAlivePlay>() {
            encoder
                .append_packet(&ServerKeepAlivePlay { id: pkt.id })
                .unwrap();
        }
    }
}

fn handle_syncrhonize_player_position(
    mut query: Query<&mut Transform, With<Player>>,
    mut encoder: ResMut<PacketEncoder>,
    mut pkts: EventReader<PacketEvent>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        for pkt in pkts.read() {
            if let Some(pkt) = pkt.decode::<SynchronizePlayerPosition>() {
                encoder
                    .append_packet(&ConfirmTeleport {
                        teleport_id: pkt.teleport_id,
                    })
                    .unwrap();

                if pkt.flags & 0x01 == 0 {
                    transform.translation.x = pkt.x as f32;
                } else {
                    transform.translation.x += pkt.x as f32;
                }
                if pkt.flags & 0x02 == 0 {
                    transform.translation.y = pkt.y as f32;
                } else {
                    transform.translation.y += pkt.y as f32;
                }
                if pkt.flags & 0x04 == 0 {
                    transform.translation.z = pkt.z as f32;
                } else {
                    transform.translation.z += pkt.z as f32;
                }

                let old_rot = transform.rotation.to_euler(EulerRot::YXZ);
                let mut yaw = pkt.yaw - 90.0;
                let mut pitch = pkt.pitch;

                if pkt.flags & 0x08 == 0 {
                    yaw += old_rot.0;
                }
                if pkt.flags & 0x10 == 0 {
                    pitch = (pitch + old_rot.1).clamp(-90.0, 90.0);
                }

                transform.rotation =
                    Quat::from_euler(EulerRot::YXZ, yaw.to_radians(), pitch.to_radians(), 0.0);
            }
        }
    }
}
