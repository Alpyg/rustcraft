use uuid::Uuid;

use crate::{define_protocol, Bounded, Decode, Encode, LenPrefixed, RawBytes, VarInt, NBT};

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
            0x01 PingStatus {
                payload: i64,
            },
        },
        Server {
            0x00 StatusRequest {},
            0x01 PongStatus {
                payload: i64,
            }
        },
    },

    Login {
        Client {
            0x00 DisconnectLogin {
                reason: &'a str,
            },
            0x01 EncryptionRequest {
                server_id: Bounded<&'a str, 20>,
                public_key: LenPrefixed<u8>,
                verify_token: LenPrefixed<u8>,
            },
            0x02 LoginSuccess {
                uuid: Uuid,
                username: &'a str,
                properties: LenPrefixed<Property>,
            },
            0x03 SetCompression {
                threshold: VarInt,
            },
            0x04 LoginPluginRequest {
                message_id: VarInt,
                channel: &'a str,
                data: RawBytes<'a>,
            },
        },
        Server {
            0x00 LoginStart {
                name: Bounded<&'a str, 16>,
                uuid: Uuid,
            },
            0x01 EncryptionResponse {
                shared_secret: LenPrefixed<u8>,
                verify_token: LenPrefixed<u8>,
            },
            0x02 LoginPluginResponse {
                message_id: VarInt,
                successful: bool,
                data: Option<Bounded<RawBytes<'a>, 1048576>>,
            },
            0x03 LoginAcknowledged {},
        },
    },

    Configuration {
        Client {
            0x00 ClientPluginMessageConfiguration {
                channel: &'a str, // Ident
                data: RawBytes<'a>,
            },
            0x01 DisconnectConfiguration {
                reason: &'a str, // Text
            },
            0x02 FinishConfiguration {},
            0x03 KeepAliveClientConfiguration {
                id: i64,
            },
            0x04 PingConfiguration {
                id: i32,
            },
            0x05 RegistryData {
                registry_codec: NBT,
            },
            0x06 RemoveResourcePackConfiguration {
                uuid: Option<Uuid>,
            },
            0x07 AddResourcePackConfiguration {
                uuid: Uuid,
                url: &'a str,
                hash: Bounded<&'a str, 40>,
                forced: bool,
                option: Option<String> // Text
            },
            0x08 FeatureFlags {
                feature_flags: LenPrefixed<String> // Ident
            },
            0x09 UpdateTagsConfiguration {
                tags: LenPrefixed<TagArray>,
            },
        },
        Server {
            0x00 ClientInformationConfiguration {
                locale: Bounded<&'a str, 16>,
                view_distance: u8,
                chat_mode: ChatMode,
                chat_colors: bool,
                displayed_skin_parts: u8,
                main_hand: Hand,
                enable_text_filtering: bool,
                allow_server_listings: bool,
            },
            0x01 PluginMessageConfiguration {
                channel: &'a str, // Ident
                data: Bounded<RawBytes<'a>, 32767>,
            },
            0x02 AcknowledgeFinishConfiguration {},
            0x03 KeepAliveServerConfiguration {
                id: i64,
            },
            0x04 PongConfiguration {
                id: i32,
            },
            0x05 ResourcePackResponseConfiguration {
                uuid: Uuid,
                result: ResourcePackResponseConfigurationResult,
            },
        },
    },
});

#[derive(Encode, Decode, Debug, Copy, Clone, PartialEq, Eq)]
pub enum HandshakeNextState {
    Status,
    Login,
}

#[derive(Encode, Decode, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Property<S = String> {
    pub name: S,
    pub value: S,
    pub signature: Option<S>,
}

#[derive(Encode, Decode, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChatMode {
    Enabled,
    CommandOnly,
    Hidden,
}

#[derive(Encode, Decode, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Encode, Decode, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResourcePackResponseConfigurationResult {
    SuccessfullyDownloaded,
    Declined,
    FailedToDownload,
    Accpeted,
    Downloaded,
    InvalidURL,
    FailedToReload,
    Discarded,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq)]
pub struct TagArray {
    identifier: String,
    tags: LenPrefixed<Tag>,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    identifier: String,
    ids: LenPrefixed<VarInt>,
}
