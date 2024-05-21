use uuid::Uuid;

#[allow(dead_code)]
use crate::define_protocol;

use crate::protocol::{profile::*, types::*, *};

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

    Status {
        Clientbound {
            0x00 StatusResponse {
                response: String,
            },
            0x01 PingResponse {
                payload: i64,
            },
        },
        Serverbound {
            0x00 StatusRequest {},
            0x01 PingRequest {
                payload: i64,
            }
        },
    },

    Login {
        Clientbound {
            0x00 Disconnect {
                reason: String,
            },
            0x01 EncryptionRequest {
                server_id: String,
                public_key_length: VarInt,
                public_key: Vec<u8>,
                verify_token_length: VarInt,
                verify_token: Vec<u8>,
            },
            0x02 LoginSuccess {
                uuid: Uuid,
                username: String,
                number_of_properties: VarInt,
                properties: Vec<Property>,
            },
            0x03 SetCompression {
                threshold: VarInt,
            },
            0x04 LoginPluginRequest {
                message_id: VarInt,
                channel: String,
                data: Vec<u8>,
            },
        },
        Serverbound {
            0x00 LoginStart {
                name: String,
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
