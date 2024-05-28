use bevy::prelude::*;
use protocol::{
    packets::{
        ConfirmTeleport, KeepAliveClientPlay, KeepAliveServerPlay, SynchronizePlayerPosition,
    },
    PacketEncoder, PacketEvent,
};

use crate::core::LocalPlayer;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_keep_alive, handle_syncrhonize_player_position),
        );
    }
}

fn handle_keep_alive(mut encoder: ResMut<PacketEncoder>, mut pkts: EventReader<PacketEvent>) {
    for pkt in pkts.read() {
        if let Some(pkt) = pkt.decode::<KeepAliveClientPlay>() {
            encoder
                .append_packet(&KeepAliveServerPlay { id: pkt.id })
                .unwrap();
        }
    }
}

fn handle_syncrhonize_player_position(
    mut query: Query<&mut Transform, With<LocalPlayer>>,
    mut encoder: ResMut<PacketEncoder>,
    mut pkts: EventReader<PacketEvent>,
) {
    let mut transform = query.get_single_mut().unwrap();
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
