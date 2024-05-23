use bevy::prelude::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, test);
    }
}

fn test() {}
