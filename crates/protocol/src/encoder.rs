use anyhow::ensure;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bytes::{BufMut, BytesMut};

use crate::{Encode, Packet, VarInt, MAX_PACKET_SIZE};

type Cipher = cfb8::Encryptor<aes::Aes128>;

#[derive(Reflect, Resource, InspectorOptions, Debug)]
#[reflect(Resource, InspectorOptions)]
pub struct PacketEncoder {
    #[reflect(ignore)]
    buf: BytesMut,
    compress_buf: Vec<u8>,
    threshold: i32,
    #[reflect(ignore)]
    cipher: Option<Cipher>,
}

impl Default for PacketEncoder {
    fn default() -> Self {
        Self {
            buf: BytesMut::default(),
            compress_buf: vec![],
            threshold: -1,
            cipher: None,
        }
    }
}

impl PacketEncoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prepend_packet<P>(&mut self, pkt: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        Ok(())
    }

    pub fn append_packet<P>(&mut self, pkt: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        let start_len = self.buf.len();
        pkt.encode_with_id(&mut self.buf)?;
        let pkt_len = self.buf.len() - start_len;

        // TODO: Add compression

        ensure!(
            pkt_len <= MAX_PACKET_SIZE as usize,
            "packet exceeds maximum length"
        );

        let pkt_len_varint = VarInt(pkt_len as i32);
        let pkt_len_size = pkt_len_varint.size();

        self.buf.put_bytes(0, pkt_len_size);
        self.buf
            .copy_within(start_len..start_len + pkt_len, start_len + pkt_len_size);

        unsafe { self.buf.set_len(start_len) };
        pkt_len_varint.encode(&mut self.buf)?;
        unsafe { self.buf.set_len(start_len + pkt_len_size + pkt_len) }

        Ok(())
    }

    pub fn take(&mut self) -> BytesMut {
        self.buf.split()
    }
}
