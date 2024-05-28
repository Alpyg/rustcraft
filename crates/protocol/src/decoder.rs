use std::fmt;

use anyhow::{ensure, Context};
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bytes::{Buf, BytesMut};

use crate::{Decode, Packet, VarInt, MAX_PACKET_SIZE};

type Cipher = cfb8::Decryptor<aes::Aes128>;

#[derive(Reflect, Resource, InspectorOptions, Debug)]
#[reflect(Resource, InspectorOptions)]
pub struct PacketDecoder {
    #[reflect(ignore)]
    pub buf: BytesMut,
    decompress_buf: Vec<u8>,
    threshold: i32,
    #[reflect(ignore)]
    cipher: Option<Cipher>,
}

impl Default for PacketDecoder {
    fn default() -> Self {
        Self {
            buf: BytesMut::default(),
            decompress_buf: vec![],
            threshold: -1,
            cipher: None,
        }
    }
}

impl PacketDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_next_packet(&mut self) -> anyhow::Result<Option<PacketFrame>> {
        let mut rdr = &self.buf[..];
        let pkt_len = match VarInt::decode(&mut rdr) {
            Ok(len) => len.0,
            Err(_) => return Ok(None),
        };
        ensure!(
            pkt_len <= MAX_PACKET_SIZE,
            "packet length of {pkt_len} is out of bounds"
        );

        if rdr.len() < pkt_len as usize {
            return Ok(None);
        }

        let pkt_len_size = VarInt(pkt_len).size();

        // TODO: Add decompression

        self.buf.advance(pkt_len_size);
        let mut data = self.buf.split_to(pkt_len as usize);

        rdr = &data[..];
        let pkt_id = VarInt::decode(&mut rdr)
            .context("failed to decode packet id")?
            .0;

        data.advance(data.len() - rdr.len());

        Ok(Some(PacketFrame {
            id: pkt_id,
            body: data,
        }))
    }

    pub fn queue_bytes(&mut self, bytes: BytesMut) {
        self.buf.unsplit(bytes);
    }

    pub fn queue_slice(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes)
    }
}

#[derive(Clone)]
pub struct PacketFrame {
    pub id: i32,
    pub body: BytesMut,
}

impl PacketFrame {
    pub fn decode<'a, P>(&'a self) -> anyhow::Result<P>
    where
        P: Packet + Decode<'a>,
    {
        ensure!(
            P::ID == self.id,
            "packet id mismatch while decoding '{}': expected {}, got {}",
            P::NAME,
            P::ID,
            self.id
        );

        let mut rdr = &self.body[..];
        let pkt = P::decode(&mut rdr)?;

        ensure!(
            rdr.is_empty(),
            "missed {} bytes while decoding '{}'",
            rdr.len(),
            P::NAME
        );

        Ok(pkt)
    }
}

impl fmt::Debug for PacketFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PacketFrame")
            .field("id", &format_args!("{:#04x}", self.id))
            .field("body", &self.body)
            .finish()
    }
}
