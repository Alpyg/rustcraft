use uuid::Uuid;

use crate::{define_protocol, Decode, Encode, Property, VarInt};

define_protocol!(765 {
    Handshaking {
        Server {
            0x00 Handshake {
                protocol_version: VarInt,
                host: &'a str,
                port: u16,
                next: u8,
            },
        },
    },

    Status {
        Client {
            0x00 StatusResponse {
                response: &'a str,
            },
            0x01 PingResponse {
                payload: i64,
            },
        },
        Server {
            0x00 StatusRequest {},
            0x01 PingRequest {
                payload: i64,
            }
        },
    },

    Login {
        Client {
            0x00 Disconnect {
                reason: &'a str,
            },
            0x01 EncryptionRequest {
                server_id: &'a str,
                public_key_length: VarInt,
                public_key: Vec<u8>,
                verify_token_length: VarInt,
                verify_token: Vec<u8>,
            },
            0x02 LoginSuccess {
                uuid: Uuid,
                username: &'a str,
                number_of_properties: VarInt,
                properties: Vec<Property>,
            },
            0x03 SetCompression {
                threshold: VarInt,
            },
            0x04 LoginPluginRequest {
                message_id: VarInt,
                channel: &'a str,
                data: Vec<u8>,
            },
        },
        Server {
            0x00 LoginStart {
                name: &'a str,
                uuid: Uuid,
            },
            0x01 EncryptionResponse {
                shared_secret_length: VarInt,
                shared_secret: Vec<u8>,
                verify_token_length: VarInt,
                verify_token: Vec<u8>,
            },
            0x02 LoginPluginResponse {
                message_id: VarInt,
                successful: bool,
                data: Option<Vec<u8>>,
            },
            0x03 LoginAcknowledged {},
        },
    },
});

#[derive(Encode, Decode, Debug, Copy, Clone, PartialEq, Eq)]
pub enum HandshakeNextState {
    Status,
    Login,
}
