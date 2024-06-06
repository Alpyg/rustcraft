use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Processing,
    #[allow(dead_code)]
    MainMenu,
    #[allow(dead_code)]
    InGame,
}
