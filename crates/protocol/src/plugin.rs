use bevy::prelude::*;

use crate::packets::Handshake;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, test);
    }
}

fn test() {
    let _packet = Handshake {
        protocol_version: 765,
        host: "localhost".to_owned(),
        port: 25565,
        next: 1,
    };
}
