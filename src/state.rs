use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    LoadingTextures,
    ProcessingTextures,
    LoadingModels,
    #[allow(dead_code)]
    MainMenu,
    #[allow(dead_code)]
    InGame,
}
