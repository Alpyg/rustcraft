use bevy::prelude::*;

use crate::{decoder::PacketDecoder, encoder::PacketEncoder};

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // TODO: only registser_type and init when connecting to a server
        app.register_type::<PacketDecoder>();
        app.init_resource::<PacketDecoder>();
        app.register_type::<PacketEncoder>();
        app.init_resource::<PacketEncoder>();
        app.add_systems(Startup, test);
    }
}

fn test() {}
