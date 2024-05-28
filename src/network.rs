use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Instant;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use bytes::{Buf, BytesMut};
use protocol::{packets::*, PacketDecoder, PacketEncoder, PacketEvent, VarInt, NBT};
use uuid::Uuid;

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
            connect.run_if(not(resource_exists::<ServerConnection>)),
        );
        app.add_systems(
            PreUpdate,
            receive_packets.run_if(resource_exists::<ServerConnection>),
        );
    }
}

/// TODO: move this to an async method so it doesnt block
fn connect(
    mut commands: Commands,
    mut encoder: ResMut<PacketEncoder>,
    mut decoder: ResMut<PacketDecoder>,
) {
    let mut stream = match TcpStream::connect("127.0.0.1:25565") {
        Ok(stream) => stream,
        Err(e) => panic!("Could not connect to the server: {e:#}"),
    };
    stream.set_nodelay(true).unwrap();

    // handshake
    encoder
        .append_packet(&Handshake {
            protocol_version: VarInt(765),
            host: "localhost",
            port: 25565,
            next: 2,
        })
        .unwrap();
    encoder
        .append_packet(&LoginStart {
            name: "Rust".into(),
            uuid: Uuid::from_u128(0),
        })
        .unwrap();
    stream.write_all(&encoder.take()).unwrap();
    stream.flush().unwrap();

    let mut buf = [0; 10000];
    let len = stream.read(&mut buf).unwrap();
    decoder.queue_slice(&buf[0..len]);
    let pkt_frame = decoder.try_next_packet().unwrap().unwrap();
    let _login_success = pkt_frame.decode::<LoginSuccess>().unwrap();

    encoder.append_packet(&LoginAcknowledged {}).unwrap();
    stream.write_all(&encoder.take()).unwrap();
    stream.flush().unwrap();

    let mut buf = [0; 50000];
    let len = stream.read(&mut buf).unwrap();
    decoder.queue_slice(&buf[0..len]);

    let _pkt_frame = decoder.try_next_packet().unwrap().unwrap(); // Clientbound Plugin Message
    let pkt_frame = decoder.try_next_packet().unwrap().unwrap(); // Feature Flags
    let _feature_flags = pkt_frame.decode::<FeatureFlags>().unwrap();

    let mut buf = [0; 50000];
    let len = stream.read(&mut buf).unwrap();
    decoder.queue_slice(&buf[0..len]);

    let _pkt_frame = decoder.try_next_packet(); // Registry Data
    let _pkt_frame = decoder.try_next_packet(); // Update Tags
    let _pkt_frame = decoder.try_next_packet(); // Finish Configuration

    encoder
        .append_packet(&AcknowledgeFinishConfiguration {})
        .unwrap();
    stream.write_all(&encoder.take()).unwrap();
    stream.flush().unwrap();

    stream.set_nonblocking(true).unwrap();
    commands.insert_resource(ServerConnection::new("127.0.0.1:25565".to_string(), stream));
}

fn receive_packets(
    mut ev: EventWriter<PacketEvent>,
    mut decoder: ResMut<PacketDecoder>,
    connection: ResMut<ServerConnection>,
) {
    let mut stream = connection.stream.as_ref().unwrap();

    loop {
        let frame = match decoder.try_next_packet() {
            Ok(Some(frame)) => frame,
            Ok(None) => {
                // Incomplete packet. Read more data.
                let mut buf = [0; 4096];
                let len = match stream.read(&mut buf) {
                    Ok(len) => len,
                    Err(e) => {
                        debug!("Error reading data from stream: {e:#}");
                        break;
                    }
                };
                if len == 0 {
                    break;
                }
                decoder.queue_slice(&buf[..len]);
                continue;
            }
            Err(e) => {
                warn!(
                    "Error decoding packet frame: {e:#}, {:?}",
                    &decoder.buf.len()
                );
                break;
            }
        };

        println!("{:#?}", frame);

        ev.send(PacketEvent {
            timestamp: Instant::now(),
            id: frame.id,
            data: frame.body.freeze(),
        });
    }
}
