use std::fs;

use bevy::{prelude::*, utils::HashMap};
use bimap::BiMap;
use serde::{Deserialize, Deserializer};

use protocol::VarInt;
use serde_json::Value;

#[derive(Deserialize, Debug, Default)]
pub struct ProtocolRegistry {
    #[serde(deserialize_with = "deserialize_registry_id")]
    protocol_id: VarInt,
    #[serde(deserialize_with = "deserialize_entries")]
    entries: BiMap<String, VarInt>,
}

#[derive(Resource, Debug)]
pub struct ProtocolRegistries {
    map: HashMap<VarInt, String>,
    registries: HashMap<String, ProtocolRegistry>,
}

fn deserialize_registry_id<'de, D>(deserializer: D) -> Result<VarInt, D::Error>
where
    D: Deserializer<'de>,
{
    let protocol_id: i32 = i32::deserialize(deserializer)?;

    Ok(VarInt(protocol_id))
}

fn deserialize_entries<'de, D>(deserializer: D) -> Result<BiMap<String, VarInt>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Entry {
        protocol_id: i32,
    }

    let raw_entries: HashMap<String, Entry> = HashMap::deserialize(deserializer)?;
    let mut entries = BiMap::new();

    for (key, entry) in raw_entries {
        entries.insert(key, VarInt(entry.protocol_id));
    }

    Ok(entries)
}

impl Default for ProtocolRegistries {
    fn default() -> Self {
        let data = fs::read_to_string("assets/reports/registries.json").unwrap();
        let value: Value = serde_json::from_str(&data).unwrap();

        let registries: HashMap<String, ProtocolRegistry> = serde_json::from_value(value).unwrap();
        let mut map = HashMap::new();

        for (name, registry) in registries.iter() {
            map.insert(registry.protocol_id, name.to_owned());
        }

        Self { map, registries }
    }
}

impl ProtocolRegistries {
    pub fn get_registry_by_name(&self, registry: &str) -> Option<&ProtocolRegistry> {
        self.registries.get(registry)
    }
    pub fn get_registry_by_id(&self, registry: &VarInt) -> Option<&ProtocolRegistry> {
        if let Some(registry) = self.map.get(registry) {
            return self.registries.get(registry);
        }
        None
    }
}

impl ProtocolRegistry {
    pub fn get_id(&self, entry: &str) -> Option<&VarInt> {
        self.entries.get_by_left(entry)
    }
    pub fn get_name(&self, entry: &VarInt) -> Option<&String> {
        self.entries.get_by_right(entry)
    }
}
