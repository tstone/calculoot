use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
#[allow(unused)]
pub enum GameMode {
    StartMenu,
    #[default]
    InGame,
}
