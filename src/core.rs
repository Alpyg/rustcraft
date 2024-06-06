#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Debug, Copy, Clone, PartialEq, Eq)]
pub struct EntityId(pub i32);

#[derive(Component, Deref, DerefMut, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Yaw(pub u8);

#[derive(Component, Deref, DerefMut, Debug, Copy, Clone, PartialEq, Eq)]
pub struct HeadYaw(pub u8);

#[derive(Component, Deref, DerefMut, Debug, Copy, Clone, PartialEq, Eq)]
pub struct HeadPitch(pub u8);

#[derive(Component, Deref, DerefMut, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Velocity(pub IVec3);

#[derive(Component)]
pub struct OnGround;

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct Player;
