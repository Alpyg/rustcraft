use bevy::reflect::Reflect;
use serde::Deserialize;

#[derive(Reflect, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    Y,
    Z,
    X,
    None,
}

impl Axis {
    pub fn as_string(&self) -> &'static str {
        match *self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
            Axis::None => "none",
        }
    }

    pub fn index(&self) -> usize {
        match *self {
            Axis::Y => 0,
            Axis::Z => 2,
            Axis::X => 1,
            Axis::None => 3,
        }
    }
}
