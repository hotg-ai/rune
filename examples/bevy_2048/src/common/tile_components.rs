//! This module contains the implementation of the components Tile and Position.

use bevy::prelude::*;

use super::GameSize;

/// Component for saving tile level.
#[derive(Debug)]
pub struct Tile {
    pub level: u32,
}

impl Tile {
    /// Each level has a unique color (up to 9).
    /// Returns the color for a given tile.
    pub fn color(&self) -> Color {
        match self.level {
            0 => Color::rgb_u8(255, 255, 0),  // Yellow
            1 => Color::rgb_u8(255, 165, 0),  // Orange
            2 => Color::rgb_u8(255, 0, 0),    // Red
            3 => Color::rgb_u8(138, 43, 226), // Blue Violet
            4 => Color::rgb_u8(0, 0, 255),    // Blue
            5 => Color::rgb_u8(0, 255, 255),  // Cyan
            6 => Color::rgb_u8(124, 252, 0),  // Lawn Green
            7 => Color::rgb_u8(0, 100, 0),    // Dark Green
            8 => Color::rgb_u8(139, 69, 19),  // Saddle Brown
            9 => Color::rgb_u8(184, 134, 11), // Dark Golden Rod
            _ => Color::BLACK,
        }
    }

    /// Calculates the score of a given tile (pow(2, level)).
    pub fn score(&self) -> u32 { 2u32.pow(self.level + 1) }
}
/// Component for saving the position of a tile in the grid.
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    /// Calculates the index of the position on a board
    /// represented by a 1D array.
    pub fn index(&self) -> usize { self.row * 4 + self.col }

    /// Transforms a position into a world point according to the board's size.
    pub fn to_vec3(self, game_size: GameSize) -> Vec3 {
        // Offset from the bottom left point of the board.
        let offset = Vec3::new(
            -(game_size.board_size() - game_size.tile_size()) / 2.0
                + game_size.tile_spacing(),
            -(game_size.board_size() - game_size.tile_size()) / 2.0
                + game_size.tile_spacing(),
            0.0,
        );

        Vec3::new(
            (game_size.tile_size() + game_size.tile_spacing())
                * self.col as f32,
            (game_size.tile_size() + game_size.tile_spacing())
                * self.row as f32,
            0.0,
        ) + offset
    }
}
