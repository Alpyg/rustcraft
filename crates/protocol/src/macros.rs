/// Define protocol macro
#[macro_export]
macro_rules! define_protocol {
    ($version:literal {
        $($state:ident {
            $($direction:ident {
                $($id:literal $name:ident {
                    $($fname:ident: $ftype:ty) ,* $(,)?
                }),* $(,)?
            }),* $(,)?
        }),* $(,)?
    }) => {
        use protocol_derive::{Encode};

        $($($(
        #[derive(Encode ,Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $(pub $fname: $ftype),*
        }

        impl Packet for $name {
            const ID: i32 = $id;
            const NAME: &'static str = stringify!($name);
            const DIRECTION: PacketDirection = PacketDirection::$direction;
            const STATE: PacketState = PacketState::$state;
        }
        )*)*)*
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

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
            Serverbound {
                0x00 StatusRequest {},
                0x01 PingRequest {
                    payload: i64,
                }
            },
            Clientbound {
                0x00 StatusResponse {
                    response: String,
                },
                0x01 PingResponse {
                    payload: i64,
                },
            },
        },
    });

    #[test]
    fn packet_metadata() {
        assert_eq!(Handshake::ID, 0x00);
        assert_eq!(Handshake::NAME, "Handshake");
        assert_eq!(Handshake::DIRECTION, PacketDirection::Serverbound);
        assert_eq!(Handshake::STATE, PacketState::Handshaking);
    }

    #[test]
    fn packet_data() {
        let packet = Handshake {
            protocol_version: 1,
            host: "localhost".to_owned(),
            port: 25565,
            next: 1,
        };

        assert_eq!(packet.protocol_version, 1);
    }
}
