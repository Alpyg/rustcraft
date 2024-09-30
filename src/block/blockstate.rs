use std::fmt;

use bevy::utils::HashMap;
use derive_more::{AsRef, Deref, DerefMut};
use indexmap::IndexMap;
use serde::{
    de,
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

#[derive(Deserialize, Debug, Default)]
pub struct BlockDefinition {
    pub definition: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub properties: IndexMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_states")]
    pub states: HashMap<i32, BlockStateDefinition>,
}

#[derive(Deserialize, Debug, Default)]
pub struct BlockStateDefinition {
    pub id: i32,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub properties: IndexMap<String, String>,
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
    #[serde(rename = "variants")]
    Variants(HashMap<String, BlockStateVariant>),
    #[serde(rename = "multipart")]
    Multipart(Vec<BlockStateMultipart>),
}

#[derive(Debug, Clone, Deref, DerefMut, AsRef)]
pub struct BlockStateVariant(pub Vec<BlockStateModel>);

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

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BlockStateModel {
    pub model: String,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub uvlock: bool,
    #[serde(default)]
    pub weight: f32,
}

#[derive(Deserialize, Debug)]
pub struct BlockStateMultipart {
    #[serde(deserialize_with = "deserialize_multipart_apply")]
    pub apply: Vec<BlockStateModel>,
    pub when: Option<BlockStateMultipartWhen>,
}

#[derive(Deserialize, Debug)]
pub enum BlockStateMultipartWhen {
    #[serde(rename = "OR")]
    Or(Vec<HashMap<String, String>>),
    #[serde(rename = "AND")]
    And(Vec<HashMap<String, String>>),
    #[serde(untagged)]
    State(HashMap<String, String>),
}

fn deserialize_multipart_apply<'de, D>(deserializer: D) -> Result<Vec<BlockStateModel>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ApplyVisitor;

    impl<'de> Visitor<'de> for ApplyVisitor {
        type Value = Vec<BlockStateModel>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a single model or a list of models")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut models = Vec::new();
            while let Some(model) = seq.next_element()? {
                models.push(model);
            }
            Ok(models)
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let model: BlockStateModel =
                Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(vec![model])
        }
    }

    deserializer.deserialize_any(ApplyVisitor)
}
