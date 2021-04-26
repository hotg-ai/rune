//! This module contains the implementation of the moving_input system.

use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use crate::common::GameState;
use super::{MovingDirection, MovingState};

/// While the moving state is `Idle`, getting the input
/// of the user.
/// If the user pressed the arrows or a,w,d,s keys,
/// the direction is being chosen
pub fn moving_input(
    game_state: Res<GameState>,
    input: Res<Arc<RwLock<Option<MovingDirection>>>>,
    mut moving_state: ResMut<MovingState>,
    mut moving_dir: ResMut<MovingDirection>,
    mut next_dir: ResMut<Option<MovingDirection>>,
) {
    if !matches!(*game_state, GameState::Play) {
        return;
    }

    let input = input.read().unwrap();

    if let Some(direction) = *input {
        if matches!(*moving_state, MovingState::Idle) {
            *moving_dir = direction;
            *moving_state = MovingState::SetMoving { starting: true };
        } else {
            *next_dir = Some(direction);
        }
    }
}

/// This system checks whether the game is idle and there is a next direction to
/// move.
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
