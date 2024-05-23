use std::default::Default;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bytes::{Bytes, BytesMut};

use protocol::{packets::*, Decode, Encode, Packet, VarInt};

#[derive(Reflect, Resource, InspectorOptions, Debug)]
#[reflect(Resource, InspectorOptions)]
pub struct ServerConnection {
    pub host: String,
    #[reflect(ignore)]
    pub stream: Option<TcpStream>, // Option for reflect to work
} //

impl ServerConnection {
    fn new(host: String, stream: TcpStream) -> Self {
        ServerConnection {
            host,
            stream: Some(stream),
        }
    }
}

impl Default for ServerConnection {
    fn default() -> Self {
        Self {
            host: "".to_owned(),
            stream: None,
        }
    }
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ServerConnection>();
        app.add_systems(
            Startup,
            status_request.run_if(resource_exists::<ServerConnection>),
        );

        if let Ok(stream) = TcpStream::connect("127.0.0.1:25565") {
            stream.set_nodelay(true).unwrap();
            app.insert_resource(ServerConnection::new("127.0.0.1:25565".to_string(), stream));
        }
    }
}

/// TODO: move this to an async method so it doesnt block
fn status_request(_commands: Commands, connection: ResMut<ServerConnection>) {
    let mut stream = connection.stream.as_ref().unwrap();
    let mut buf = BytesMut::new();
    // handshake
    let handshake = Handshake {
        protocol_version: VarInt(765),
        host: "localhost".to_owned(),
        port: 25565,
        next: 1,
    };

    let _ = VarInt(16).encode(&mut buf); // length
    let _ = VarInt(Handshake::ID).encode(&mut buf); // id
    let _ = handshake.encode(&mut buf);

    // status
    let _ = VarInt(1).encode(&mut buf); // length
    let _ = VarInt(0).encode(&mut buf); // id 1

    stream.write_all(&buf).unwrap();
    stream.flush().unwrap();

    let mut buf2 = [0; 512];
    let _len = stream.read(&mut buf2[..]).unwrap();

    let mut buf2 = Bytes::copy_from_slice(&buf2);
    let _packet_len = VarInt::decode(&mut buf2).unwrap().0;
    let _packet_id = VarInt::decode(&mut buf2).unwrap();
    let response_len = VarInt::decode(&mut buf2).unwrap().0 as usize;
    println!("{}", str::from_utf8(&buf2[0..(response_len)]).unwrap());
}
