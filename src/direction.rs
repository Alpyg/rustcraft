use bevy::reflect::Reflect;
use serde::Deserialize;

#[derive(Reflect, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}
