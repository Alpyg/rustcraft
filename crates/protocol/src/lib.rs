use anyhow::Context;
use bytes::BytesMut;

use protocol_derive::{Decode, Encode};

pub mod __private {
    pub use anyhow::{anyhow, bail, ensure, Context, Result};

    pub use crate::{Decode, Encode, Packet, VarInt};
}

mod impls;
mod macros;
pub mod packets;
mod plugin;

pub use impls::*;
pub use plugin::*;
//pub use protocol_derive::{Decode, Encode};

extern crate self as protocol;

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

    fn encode_with_id(&self, wtr: &mut BytesMut) -> anyhow::Result<()>
    where
        Self: Encode,
    {
        VarInt(Self::ID)
            .encode(wtr)
            .context("failed to encode packet ID")?;
        self.encode(wtr)
    }
}

pub trait Encode {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()>;
}

pub trait Decode<'a>: Sized {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self>;
}

#[derive(Encode)]
pub struct TestPacket {
    pub a: u8,
    pub b: u8,
}
