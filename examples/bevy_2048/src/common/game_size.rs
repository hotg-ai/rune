//! This module contains the implementation of GameSize.
use bevy::prelude::*;

use crate::{
    board::{Board, EmptyTile},
    movement::Moving,
    tile_spawning::SpawnAnimation,
};

use super::{Position, Tile};
/// This plugin builds the game size systems and resource into the app.
pub struct GameSizePlugin;

impl Plugin for GameSizePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<GameSize>()
            .add_system(update_game_size.system())
            .add_system(update_board_size.system())
            .add_system(update_tiles_size_and_position.system());
    }
}

/// A struct that gives the size of the game's components ratioed by the board
/// size.
#[derive(Debug, Copy, Clone)]
pub struct GameSize(f32);

impl GameSize {
    /// Returns the board's size.
    pub fn board_size(&self) -> f32 { self.0 }

    /// Calculates the tiles' size.
    pub fn tile_size(&self) -> f32 { (self.0 * 0.85) / 4.0 }

    /// Calculates the space between two tiles.
    pub fn tile_spacing(&self) -> f32 { (self.0 * 0.15) / 5.0 }

    /// Calculates the amount that the tile should get increased by when a merge
    /// occur.
    pub fn merge_size(&self) -> f32 { self.tile_size() * 0.1 }

    /// Gets the window size and calculates the game size.
    fn calculate_game_size(&mut self, width: f32, height: f32) {
        let (width, height) = (width * 0.9, height * 0.9);
        self.0 = height.min(width * 0.6);
    }
}

impl Default for GameSize {
    /// Creates a game size based on a board with a size of 500.
    fn default() -> Self { Self(500.0) }
}

/// This system updates the game size according to the window size.
pub fn update_game_size(
    mut game_size: ResMut<GameSize>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    game_size
        .calculate_game_size(window.width() as f32, window.height() as f32);
}

/// This system updates the board (background) size.
pub fn update_board_size(
    game_size: Res<GameSize>,
    mut sprite: Mut<Sprite>,
    _: &Board,
) {
    sprite.size = Vec2::new(game_size.board_size(), game_size.board_size());
}

/// This system updates the size and position for the tiles and empty-tiles.
pub fn update_tiles_size_and_position(
    game_size: Res<GameSize>,
    mut tiles_size: Query<With<Tile, Without<SpawnAnimation, &mut Sprite>>>,
    mut tiles_position: Query<(&mut Transform, &Position, &Option<Moving>)>,
    mut empty_tiles_size: Query<With<EmptyTile, &mut Sprite>>,
    mut empty_tiles_position: Query<
        With<EmptyTile, (&mut Transform, &Position)>,
    >,
) {
    // Update the size for all the tiles.
    for mut sprite in tiles_size.iter_mut() {
        sprite.size = Vec2::new(game_size.tile_size(), game_size.tile_size());
    }

    // Update the position for all the tiles.
    for (mut transform, position, moving) in tiles_position.iter_mut() {
        if moving.is_none() {
            transform.translation = position.to_vec3(*game_size);
        }
    }

    // Update the size for all the empty-tiles.
    for mut sprite in empty_tiles_size.iter_mut() {
        sprite.size = Vec2::new(game_size.tile_size(), game_size.tile_size());
    }

    // Update the position for all the empty-tiles.
    for (mut transform, position) in empty_tiles_position.iter_mut() {
        transform.translation = position.to_vec3(*game_size);
    }
}
