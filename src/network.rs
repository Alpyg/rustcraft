use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use protocol::{packets::*, PacketDecoder, PacketEncoder, VarInt};

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
fn status_request(
    _commands: Commands,
    mut encoder: ResMut<PacketEncoder>,
    mut decoder: ResMut<PacketDecoder>,
    connection: ResMut<ServerConnection>,
) {
    let mut stream = connection.stream.as_ref().unwrap();

    // handshake
    encoder
        .append_packet(&Handshake {
            protocol_version: VarInt(765),
            host: "localhost",
            port: 25565,
            next: 1,
        })
        .unwrap();

    // status
    encoder.append_packet(&StatusRequest {}).unwrap();

    stream.write_all(&encoder.take()).unwrap();
    stream.flush().unwrap();

    let mut response_buf: [u8; 512] = [0; 512];
    let _len = stream.read(&mut response_buf).unwrap();

    decoder.queue_slice(&response_buf);

    let pkt_frame = decoder.try_next_packet().unwrap().unwrap();
    let status_response = pkt_frame.decode::<StatusResponse>().unwrap();
    println!("{:?}", &status_response);
}
