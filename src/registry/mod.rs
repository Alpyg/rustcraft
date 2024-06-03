use bevy::prelude::*;

use item_registry::ItemProtocolRegistry;
use protocol::VarInt;

use self::block_registry::BlockProtocolRegistry;

mod block_registry;
mod item_registry;

trait Registry<T> {
    fn get(&self, k: &str) -> Option<&T>;
    fn get_from_id(&self, k: &VarInt) -> Option<&T>;
    fn get_from_ident(&self, k: &str) -> Option<&T>;
}

pub struct RegistryPlugin;
impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemProtocolRegistry>();
        app.init_resource::<BlockProtocolRegistry>();
    }
}
