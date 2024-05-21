use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use bevy::prelude::*;
use bytes::{BufMut, Bytes, BytesMut};

use crate::protocol::{types::VarInt, Decode, Encode};

#[derive(Resource, Debug)]
pub struct ServerConnection {
    pub server: String,
    pub stream: TcpStream,
}

impl ServerConnection {
    fn new(server: String, stream: TcpStream) -> Self {
        ServerConnection { server, stream }
    }
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
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
fn status_request(_commands: Commands, mut connection: ResMut<ServerConnection>) {
    let mut buf = BytesMut::new();
    // handshake
    VarInt(16).encode(&mut buf); // length
    VarInt(0).encode(&mut buf); // id 1
    VarInt(765).encode(&mut buf); // protocol version 2
    VarInt(9).encode(&mut buf); // string len 1
    buf.put("localhost".as_bytes()); // host 9
    buf.put_u16(25565); // port 2
    VarInt(1).encode(&mut buf); // next 1

    // status
    VarInt(1).encode(&mut buf); // length
    VarInt(0).encode(&mut buf); // id 1

    connection.stream.write(&buf).unwrap();
    let _ = connection.stream.flush().unwrap();

    let mut buf2 = [0; 512];
    let _ = connection.stream.read(&mut buf2[..]).unwrap();

    let mut buf2 = Bytes::copy_from_slice(&buf2);
    let _packet_len = VarInt::decode(&mut buf2).unwrap().0;
    let _packet_id = VarInt::decode(&mut buf2).unwrap();
    let response_len = VarInt::decode(&mut buf2).unwrap().0 as usize;
    println!("{}", str::from_utf8(&buf2[0..(response_len)]).unwrap());
}
