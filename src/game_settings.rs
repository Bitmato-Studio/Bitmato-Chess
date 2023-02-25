
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum LogicalGameState {
    #[default]
    Splash,  // Initial splash screen
    Menu,    // the home memu
    Lobby,   // the lobby selection screen
    Loading, // loading screen
    Game,    // the actual game play screen
    Error,   // Something went wrong
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}