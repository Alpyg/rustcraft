use crate::{Decode, Encode};

#[derive(Encode, Decode, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Property<S = String> {
    pub name: S,
    pub value: S,
    pub signature: Option<S>,
}
