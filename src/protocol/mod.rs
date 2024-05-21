#![allow(dead_code)]

use bevy::prelude::*;
use bytes::{Bytes, BytesMut};

use crate::protocol::versions::v1_20_4::*;

pub mod encode;
pub mod profile;
pub mod types;
pub mod versions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    Clientbound,
    Serverbound,
}

impl PacketDirection {
    pub fn opposite(&self) -> Self {
        use PacketDirection::*;
        match self {
            Clientbound => Serverbound,
            Serverbound => Clientbound,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

impl PacketState {
    pub fn name(&self) -> String {
        use PacketState::*;
        match self {
            Handshaking => "Handshaking",
            Status => "Status",
            Login => "Login",
            Configuration => "Configuration",
            Play => "Play",
        }
        .to_owned()
    }
}

pub trait Packet: std::fmt::Debug {
    const ID: i32;

    const NAME: &'static str;

    const DIRECTION: PacketDirection;

    const STATE: PacketState;
}

pub trait Encode {
    fn encode(&self, wtr: &mut BytesMut);
}

pub trait Decode: Sized {
    fn decode(rdr: &mut Bytes) -> anyhow::Result<Self>;
}

#[derive(Debug, Resource)]
pub struct ProtocolBuffer {}

impl Default for ProtocolBuffer {
    fn default() -> Self {
        ProtocolBuffer {}
    }
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProtocolBuffer>();
        app.add_systems(Startup, test);
    }
}

fn test(_protocol_buf: ResMut<ProtocolBuffer>) {
    let _packet = Handshake {
        protocol_version: 765,
        host: "localhost".to_owned(),
        port: 25565,
        next: 1,
    };
}
