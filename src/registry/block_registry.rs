use std::collections::HashMap;

use bevy::prelude::*;
use bimap::BiMap;

use protocol::VarInt;

use super::Registry;

#[derive(Resource)]
pub struct BlockProtocolRegistry {
    map: BiMap<String, VarInt>,
    data: HashMap<String, u8>,
    id: VarInt,
}

impl Default for BlockProtocolRegistry {
    fn default() -> Self {
        let (map, id) = BlockProtocolRegistry::build_map().unwrap();
        let data = BlockProtocolRegistry::build_data().unwrap();

        //println!("{:#?} {:#?} {:#?}", map.len(), data.len(), id.0);

        Self { map, data, id }
    }
}

impl Registry<u8> for BlockProtocolRegistry {
    #[inline]
    fn get(&self, k: &str) -> Option<&u8> {
        self.data.get(k)
    }

    #[inline]
    fn get_from_id(&self, k: &VarInt) -> Option<&u8> {
        self.data.get(self.map.get_by_right(k).unwrap())
    }

    #[inline]
    fn get_from_ident(&self, k: &str) -> Option<&u8> {
        self.data.get(k)
    }
}

impl BlockProtocolRegistry {
    fn build_map() -> anyhow::Result<(BiMap<String, VarInt>, VarInt)> {
        Ok((BiMap::new(), VarInt(0)))
    }

    fn build_data() -> anyhow::Result<HashMap<String, u8>> {
        Ok(HashMap::new())
    }
}
