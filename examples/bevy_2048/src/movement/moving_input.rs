//! This module contains the implementation of the moving_input system.
use bevy::prelude::*;
use std::convert::TryFrom;

use crate::common::GameState;
use super::{MovingDirection, MovingState};

/// While the moving state is `Idle`, getting the input
/// of the user.
/// If the user pressed the arrows or a,w,d,s keys,
/// the direction is being chosen
pub fn moving_input(
    game_state: Res<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut moving_state: ResMut<MovingState>,
    mut moving_dir: ResMut<MovingDirection>,
    mut next_dir: ResMut<Option<MovingDirection>>,
) {
    if matches!(*game_state, GameState::Play) {
        // Iterating through the keys that were just pressed by the user.
        for key in keyboard_input.get_just_pressed() {
            // Checking if the keys can be converted into a direction
            if let Ok(direction) = MovingDirection::try_from(key) {
                if matches!(*moving_state, MovingState::Idle) {
                    // Setting the direction.
                    *moving_dir = direction;
                    // Setting the moving state to `SetMoving` with starting.
                    *moving_state = MovingState::SetMoving { starting: true };
                } else {
                    // If in the middle of moving, save the next direction.
                    *next_dir = Some(direction);
                }
            }
        }
    }
}

/// This system checks whether the game is idle and there is a next direction to move.
pub fn next_direction(
    mut moving_state: ResMut<MovingState>,
    mut moving_dir: ResMut<MovingDirection>,
    mut next_dir: ResMut<Option<MovingDirection>>,
) {
    if matches!(*moving_state, MovingState::Idle) {
        if let Some(direction) = *next_dir {
            *next_dir = None;

            // Moving to the next direction.
            *moving_dir = direction;
            *moving_state = MovingState::SetMoving { starting: true };
        }
    }
}
