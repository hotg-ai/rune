//! This module contains the implementation of the enum GameState.

// This enum tells in what state the game is in.
#[derive(Debug)]
pub enum GameState {
    Play,
    GameOver,
    Restarting,
}

impl Default for GameState {
    /// Creates a Play game state.
    fn default() -> Self { Self::Play }
}
