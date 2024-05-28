use std::time::Instant;

use bevy::prelude::*;
use bytes::Bytes;

use crate::{decoder::PacketDecoder, encoder::PacketEncoder};

pub struct ProtocolPlugin;
impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // TODO: only registser_type and init when connecting to a server
        app.register_type::<PacketDecoder>();
        app.init_resource::<PacketDecoder>();
        app.register_type::<PacketEncoder>();
        app.init_resource::<PacketEncoder>();
        app.add_event::<PacketEvent>();
    }
}

#[derive(Event, Debug)]
pub struct PacketEvent {
    pub timestamp: Instant,
    pub id: i32,
    pub data: Bytes,
}
