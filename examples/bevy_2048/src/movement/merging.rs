//! This module contains the implementation of the Merging state's system.

use bevy::prelude::*;

use crate::{
    common::{Position, Tile},
    score::Score,
    tile_spawning::Despawn,
};

use super::{MergeAnimation, Merged, MovingState};

/// When the moving state is `Merging`, it merging tiles
/// that are in the same position.
pub fn merging(
    mut commands: Commands,
    mut moving_state: ResMut<MovingState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut score: ResMut<Score>,
    mut tiles: Query<(
        Entity,
        &mut Tile,
        &Position,
        &mut Option<Merged>,
        &mut Handle<ColorMaterial>,
    )>,
) {
    if matches!(*moving_state, MovingState::Merging) {
        // Create a board with entity and position to check
        // if two tiles are at the same position.
        let mut board = [None; 16];
        for (entity, mut tile, position, mut merged, mut material) in tiles.iter_mut() {
            // Check if a tile is already exists at that position.
            if let Some((existing_entity, _position)) = board[position.index()] {
                // Despawning the existing tile.
                commands.despawn(existing_entity);

                // Checking that the level is not the last one.
                if tile.level < 9 {
                    // Updating current tile level and color.
                    tile.level += 1;
                    *material = materials.add(tile.color().into());

                    // Updating the score.
                    score.0 += tile.score();

                    // Setting the tile as merged.
                    *merged = Some(Merged);
                    commands.insert_one(entity, MergeAnimation::default());
                } else {
                    // If the level is the last, despawn the tile with an animation.
                    commands.insert_one(entity, Despawn);
                }
            }

            // Move the tile into the board.
            board[position.index()] = Some((entity, position));
        }

        *moving_state = MovingState::SetMoving { starting: false };
    }
}
