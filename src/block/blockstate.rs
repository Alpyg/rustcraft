use std::fmt;

use bevy::{prelude::*, utils::HashMap};
use indexmap::IndexMap;
use serde::{
    de,
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

#[derive(Resource, Debug, Default)]
pub struct BlockStateRegistry {
    pub block_definition: HashMap<String, BlockDefinition>,
    pub blockstate: HashMap<i32, BlockState>,
}

#[derive(Deserialize, Debug, Default)]
pub struct BlockDefinition {
    definition: HashMap<String, serde_json::Value>,
    #[serde(default)]
    properties: IndexMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_states")]
    states: HashMap<i32, BlockStateDefinition>,
}

#[derive(Deserialize, Debug, Default)]
pub struct BlockStateDefinition {
    id: i32,
    #[serde(default)]
    default: bool,
    #[serde(default)]
    properties: IndexMap<String, String>,
}

fn deserialize_states<'de, D>(
    deserializer: D,
) -> Result<HashMap<i32, BlockStateDefinition>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_states: Vec<BlockStateDefinition> = Vec::deserialize(deserializer)?;
    let mut states = HashMap::new();

    for state in raw_states {
        states.insert(state.id, state);
    }

    Ok(states)
}

#[derive(Deserialize, Debug)]
pub enum BlockState {
    Variant(BlockStateVariant),
    Multipart(BlockStateMultipart),
}

#[derive(Debug)]
pub struct BlockStateVariant(Vec<BlockStateModel>);

impl<'de> Deserialize<'de> for BlockStateVariant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BlockStateVariantsVisitor;

        impl<'de> Visitor<'de> for BlockStateVariantsVisitor {
            type Value = BlockStateVariant;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a BlockStateVariant or a list of BlockStateVariant")
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut variant =
                    BlockStateModel::deserialize(de::value::MapAccessDeserializer::new(map))?;
                variant.weight = 1.0;
                Ok(BlockStateVariant(vec![variant]))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut variants = Vec::<BlockStateModel>::deserialize(
                    de::value::SeqAccessDeserializer::new(seq),
                )?;
                let weight = 1.0 / variants.len() as f32;

                for variant in &mut variants {
                    if variant.weight == 0.0 {
                        variant.weight = weight;
                    }
                }

                Ok(BlockStateVariant(variants))
            }
        }

        deserializer.deserialize_any(BlockStateVariantsVisitor)
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct BlockStateModel {
    model: String,
    #[serde(default)]
    x: i16,
    #[serde(default)]
    y: i16,
    #[serde(default)]
    uvlock: bool,
    #[serde(default)]
    weight: f32,
}

#[derive(Deserialize, Debug, Default)]
pub struct BlockStateMultipart {}
