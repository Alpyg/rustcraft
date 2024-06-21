#[allow(dead_code)]
use uuid::Uuid;

use crate::{
    define_protocol, Bounded, Decode, Encode, LenPrefixed, Position, RawBytes, VarInt, NBT,
};

define_protocol!(767 {
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
                should_authenticate: bool,
            },
            0x02 LoginSuccess {
                uuid: Uuid,
                username: &'a str,
                properties: LenPrefixed<Property>,
                strict_error_handing: bool,
            },
            0x03 SetCompression {
                threshold: VarInt,
            },
            0x04 LoginPluginRequest {
                message_id: VarInt,
                channel: &'a str,
                data: RawBytes<'a>,
            },
            0x05 CookieRequestLogin {
                key: &'a str,
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
            0x04 CookieResponseLogin {
                key: &'a str,
                payload: Option<LenPrefixed<u8>>,
            }
        },
    },

    Configuration {
        Client {
            0x00 CookieRequest {
                key: &'a str,
            },
            0x01 ClientPluginMessageConfiguration {
                channel: &'a str, // Ident
                data: Bounded<RawBytes<'a>, 1048576>,
            },
            0x02 DisconnectConfiguration {
                reason: &'a str, // Text
            },
            0x03 FinishConfiguration {},
            0x04 KeepAliveClientConfiguration {
                id: i64,
            },
            0x05 PingConfiguration {
                id: i32,
            },
            0x06 ResetChat {},
            0x07 RegistryData {
                registry_codec: NBT,
            },
            0x08 RemoveResourcePackConfiguration {
                uuid: Option<Uuid>,
            },
            0x09 AddResourcePackConfiguration {
                uuid: Uuid,
                url: &'a str,
                hash: Bounded<&'a str, 40>,
                forced: bool,
                option: Option<String> // Text
            },
            0x0a StoreCookieConfiguration {
                key: &'a str,
                payload: RawBytes<'a>,
            },
            0x0b TransferConfiguration {
                host: &'a str,
                port: VarInt,
            },
            0x0c FeatureFlags {
                feature_flags: LenPrefixed<String> // Ident
            },
            0x0d UpdateTagsConfiguration {
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
            0x01 CookieResponseConfiguration {
                key: &'a str,
                payload: Option<LenPrefixed<u8>>,
            },
            0x02 ServerPluginMessageConfiguration {
                channel: &'a str, // Ident
                data: Bounded<RawBytes<'a>, 32767>,
            },
            0x03 AcknowledgeFinishConfiguration {},
            0x04 ServerKeepAliveConfiguration {
                id: i64,
            },
            0x05 PongConfiguration {
                id: i32,
            },
            0x06 ResourcePackResponseConfiguration {
                uuid: Uuid,
                result: ResourcePackResponseConfigurationResult,
            },
            0x07 ServerKnownPacks {
                count: VarInt,
                known_packs: Vec<KnownPack<'a>>,
            },
        },
    },

    Play {
        Client {
            0x24 ClientKeepAlivePlay {
                id: i64,
            },
            0x25 ChunkDataAndUpdateLight {
                chunk_x: i32,
                chunk_z: i32,
                heightmaps: NBT,
                data: LenPrefixed<u8>,
                block_entities: LenPrefixed<(u8, u16, VarInt, NBT)>, // BlockEntity
                sky_light_mask: LenPrefixed<i64>,
                block_light_mask: LenPrefixed<i64>,
                empty_sky_light_mask: LenPrefixed<i64>,
                empty_block_light_mask: LenPrefixed<i64>,
                sky_lights: LenPrefixed<LenPrefixed<u8>>,
                block_lights: LenPrefixed<LenPrefixed<u8>>,
            },
            0x29 LoginPlay {
                id: i32,
                is_hardcore: bool,
                dimensions: LenPrefixed<String>,
                max_players: VarInt,
                view_distance: VarInt,
                simulation_distance: VarInt,
                reduced_debug_info: bool,
                enable_respawn_screen: bool,
                do_limited_craftingn: bool,
                dimension_type: &'a str,
                dimension_name: &'a str,
                hashed_seed: i64,
                gamemode: u8,
                previous_gamemode: i8,
                is_debug: bool,
                is_flat: bool,
                death_location: Option<(String, Position)>,
            },
            0x3e SynchronizePlayerPosition {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                flags: u8,
                teleport_id: VarInt,
            },
        },
        Server {
            0x00 ConfirmTeleport {
                teleport_id: VarInt,
            },
            0x15 ServerKeepAlivePlay {
                id: i64,
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
pub struct KnownPack<'a> {
    namespace: &'a str,
    id: &'a str,
    version: &'a str,
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
