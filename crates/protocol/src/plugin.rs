use std::time::Instant;

use bevy::prelude::*;
use bytes::Bytes;

use crate::{decoder::PacketDecoder, encoder::PacketEncoder, Decode, Packet, ProtocolRegistries};

pub struct ProtocolPlugin;
impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // TODO: only registser_type and init when connecting to a server
        app.init_resource::<ProtocolRegistries>();
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

impl PacketEvent {
    #[inline]
    pub fn decode<'a, P>(&'a self) -> Option<P>
    where
        P: Packet + Decode<'a>,
    {
        if self.id == P::ID {
            let mut r = &self.data[..];

            match P::decode(&mut r) {
                Ok(pkt) => {
                    if r.is_empty() {
                        return Some(pkt);
                    }

                    warn!(
                        "missed {} bytes while decoding packet {} (ID = {})",
                        r.len(),
                        P::NAME,
                        P::ID
                    );
                    debug!("complete packet after partial decode: {pkt:?}");
                }
                Err(e) => {
                    warn!("failed to decode packet with ID of {}: {e:#}", P::ID);
                }
            }
        }

        None
    }
}
