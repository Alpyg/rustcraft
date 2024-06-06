#![allow(dead_code)]
use core::fmt;
use std::{collections::HashMap, fs};

use bevy::prelude::*;
use bimap::BiMap;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::{Map, Value};

use protocol::VarInt;

use super::Registry;

#[derive(Resource)]
pub struct ItemProtocolRegistry {
    bimap: BiMap<String, VarInt>,
    data: HashMap<String, Item>,
    id: VarInt,
}

impl Default for ItemProtocolRegistry {
    fn default() -> Self {
        let (bimap, id) = ItemProtocolRegistry::build_map().unwrap();
        let data = ItemProtocolRegistry::build_data().unwrap();

        Self { bimap, data, id }
    }
}

impl Registry<Item> for ItemProtocolRegistry {
    #[inline]
    fn get(&self, k: &str) -> Option<&Item> {
        self.data.get(k)
    }

    #[inline]
    fn get_from_id(&self, k: &VarInt) -> Option<&Item> {
        self.data.get(self.bimap.get_by_right(k).unwrap())
    }

    #[inline]
    fn get_from_ident(&self, k: &str) -> Option<&Item> {
        self.data.get(k)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Items {
    components: HashMap<String, Item>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    #[serde(rename = "minecraft:attribute_modifiers")]
    attribute_modifiers: AttributeModifiers,
    #[serde(rename = "minecraft:enchantments")]
    enchantments: Enchantments,
    #[serde(rename = "minecraft:lore")]
    lore: Vec<String>,
    #[serde(rename = "minecraft:damage")]
    damage: Option<u32>,
    #[serde(rename = "minecraft:max_damage")]
    max_damage: Option<u32>,
    #[serde(rename = "minecraft:max_stack_size")]
    max_stack_size: u8,
    #[serde(rename = "minecraft:rarity")]
    rarity: String,
    #[serde(rename = "minecraft:repair_cost")]
    repair_cost: u32,
    #[serde(rename = "minecraft:tool")]
    tool: Option<Tool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AttributeModifiers {
    modifiers: Vec<Modifier>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Modifier {
    #[serde(rename = "type")]
    modifier_type: String,
    amount: f64,
    id: String,
    operation: String,
    slot: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Enchantments {
    levels: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tool {
    damage_per_block: Option<u32>,
    rules: Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rule {
    #[serde(deserialize_with = "string_or_vec")]
    blocks: Vec<String>,
    correct_for_drops: Option<bool>,
    speed: Option<f64>,
}

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVecVisitor;

    impl<'de> Visitor<'de> for StringOrVecVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_string()])
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value])
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();

            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }

            Ok(vec)
        }
    }

    deserializer.deserialize_any(StringOrVecVisitor)
}

impl ItemProtocolRegistry {
    fn build_map() -> anyhow::Result<(BiMap<String, VarInt>, VarInt)> {
        #[derive(Serialize, Deserialize, Debug)]
        struct Items {
            entries: HashMap<String, ItemEntry>,
            protocol_id: i32,
        }
        #[derive(Serialize, Deserialize, Debug)]
        struct ItemEntry {
            protocol_id: i32,
        }

        let data = fs::read_to_string("assets/reports/registries.json")?;
        let val: Value = serde_json::from_str(&data)?;
        let items: Items = serde_json::from_value(val["minecraft:item"].clone())?;

        let mut bimap = BiMap::new();
        for (key, val) in items.entries.iter() {
            bimap.insert(key.to_owned(), VarInt(val.protocol_id));
        }

        Ok((bimap, VarInt(items.protocol_id)))
    }

    fn build_data() -> anyhow::Result<HashMap<String, Item>> {
        let data = fs::read_to_string("assets/reports/items.json")?;
        let val: Value = serde_json::from_str(&data)?;
        let items: Map<String, Value> = serde_json::from_value(val.clone())?;

        let mut map = HashMap::new();
        for (key, val) in items {
            let item: Item = serde_json::from_value(val.get("components").unwrap().clone())?;
            map.insert(key, item);
        }
        Ok(map)
    }
}
