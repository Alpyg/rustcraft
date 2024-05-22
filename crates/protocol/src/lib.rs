use bytes::{Bytes, BytesMut};

mod macros;
mod packets;
mod plugin;
mod types;

pub use plugin::*;
pub use types::*;

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
