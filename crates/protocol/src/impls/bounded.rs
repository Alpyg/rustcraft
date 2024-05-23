use std::{borrow::Borrow, fmt::Display};

use derive_more::{AsRef, Deref, DerefMut, From};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut, AsRef, From,
)]
pub struct Bounded<T, const MAX: usize>(pub T);

impl<T, const MAX: usize> Bounded<T, MAX> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Bounded<U, MAX> {
        Bounded(f(self.0))
    }

    pub fn map_into<U: From<T>>(self) -> Bounded<U, MAX> {
        Bounded(self.0.into())
    }
}

impl<T, const MAX: usize> Borrow<T> for Bounded<T, MAX> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T: Display, const MAX: usize> Display for Bounded<T, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
