use bevy::{math::U8Vec3, prelude::*, utils::HashMap};
use derive_more::derive::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::block::BlockId;

#[derive(Component, Debug, Clone, Serialize, Deserialize, Deref, DerefMut)]
pub struct Chunk(HashMap<U8Vec3, BlockId>);

impl Chunk {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
