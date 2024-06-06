use bevy::prelude::*;
use protocol::{packets::ChunkDataAndUpdateLight, PacketEvent};

mod chunk;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_chunk_data_and_update_light);
    }
}

fn handle_chunk_data_and_update_light(mut pkts: EventReader<PacketEvent>) {
    for pkt in pkts.read() {
        if let Some(_pkt) = pkt.decode::<ChunkDataAndUpdateLight>() {
            //println!("{:?}", pkt.data);
        }
    }
}
