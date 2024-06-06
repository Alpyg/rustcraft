#![allow(dead_code)]
use bevy::{math::Vec3, reflect::Reflect};
use serde::Deserialize;

#[derive(Reflect, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    #[default]
    X,
    Y,
    Z,
}

impl Axis {
    pub fn as_string(&self) -> &'static str {
        match *self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        }
    }

    pub fn index(&self) -> usize {
        match *self {
            Axis::Y => 0,
            Axis::Z => 2,
            Axis::X => 1,
        }
    }
}

impl From<Axis> for Vec3 {
    fn from(value: Axis) -> Self {
        match value {
            Axis::X => Vec3::X,
            Axis::Y => Vec3::Y,
            Axis::Z => Vec3::Z,
        }
    }
}
