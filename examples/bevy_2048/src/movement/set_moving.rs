//! This module contains the implementation of the SetMoving state's system.

use crate::common::{Position, Tile};
use bevy::prelude::*;

use super::{Merged, Moving, MovingDirection, MovingState};

// When the moving state is `SetMoving`, it checks which tile should move.
pub fn set_moving(
    mut commands: Commands,
    mut moving_state: ResMut<MovingState>,
    moving_dir: Res<MovingDirection>,
    tiles: Query<(Entity, &Tile, &Position, &Option<Moving>, &Option<Merged>)>,
) {
    // Checking the moving state.
    if let MovingState::SetMoving { starting } = *moving_state {
        // Creating a board represented by a 1D array
        // in order to check the neighbors tiles.
        let mut tiles = tiles.iter();
        let mut board: [Option<(
            Entity,
            &Tile,
            &Position,
            &Option<Moving>,
            &Option<Merged>,
        )>; 16] = Default::default();
        for tile in &mut tiles {
            let position = tile.2;
            board[position.index()] = Some(tile);
        }

        // Vec of all the entities that should move.
        let mut moving_entities = Vec::new();

        // Iterate on the board according to the movement direction.
        for curr_pos in moving_dir.board_iteration().iter() {
            // Checking that a tile exists in the current position.
            if let Some(curr_tile) = &board[curr_pos.index()] {
                // Checking that the new position is not out of bounds.
                if let Some(new_pos) = moving_dir.moved_position(curr_pos) {
                    // Checking if the new position contains a tile.
                    if let Some(existing_tile) = &board[new_pos.index()] {
                        // If the existing tile is moving
                        // or has the same level while both tiles are not
                        // merged, move the current
                        // tile.
                        if moving_entities.contains(&existing_tile.0)
                            || (curr_tile.1.level == existing_tile.1.level
                                && curr_tile.4.is_none()
                                && existing_tile.4.is_none())
                        {
                            moving_entities.push(curr_tile.0);
                        }
                    } else {
                        // If the new position is emtpy, move the current tile.
                        moving_entities.push(curr_tile.0);
                    }
                }
            }
        }

        let moving = !moving_entities.is_empty();

        // Set the tiles that should move to `Moving`.
        for entity in moving_entities {
            commands.insert_one(entity, Some(Moving));
        }

        *moving_state = if moving {
            MovingState::Animating
        } else {
            MovingState::Finishing { moved: !starting }
        };
    }
}
