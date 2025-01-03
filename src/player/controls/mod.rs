use bevy::prelude::*;

pub mod survival;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(survival::plugin);
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);
