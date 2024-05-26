#![feature(array_try_from_fn)]

use anyhow::Context;
use bytes::BytesMut;

pub mod __private {
    pub use anyhow::{anyhow, bail, ensure, Context, Result};

    pub use crate::{Decode, Encode, Packet, PacketSide, PacketState, VarInt};
}

mod decoder;
mod encoder;
mod impls;
mod nbt;
pub mod packets;
mod plugin;

pub use decoder::*;
pub use encoder::*;
pub use impls::*;
pub use nbt::*;
pub use plugin::*;
use protocol_derive::{define_protocol, Decode, Encode, Packet};

extern crate self as protocol;

pub const MAX_PACKET_SIZE: i32 = 2097152;
pub const MAX_DATA_LEN: usize = 1048576;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketSide {
    Client,
    Server,
}

impl PacketSide {
    pub fn opposite(&self) -> Self {
        use PacketSide::*;
        match self {
            Client => Server,
            Server => Client,
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

    const SIDE: PacketSide;

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

    fn encode_slice(slice: &[Self], wtr: &mut BytesMut) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        for v in slice {
            v.encode(wtr)?;
        }

        Ok(())
    }
}

pub trait Decode<'a>: Sized {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self>;
}
