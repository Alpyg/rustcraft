#[allow(dead_code)]
use crate::define_protocol;

define_protocol!(765 {
    Handshaking {
        Serverbound {
            0x00 Handshake {
                protocol_version: u32,
                host: String,
                port: u16,
                next: u8,
            },
        },
    },
});
