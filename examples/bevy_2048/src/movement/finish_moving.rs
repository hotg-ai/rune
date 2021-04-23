//! This module contains the implementation of the Finishing state's system.

use bevy::prelude::*;

use crate::tile_spawning::SpawnTileEvent;

use super::{Merged, MovingState};

/// When the moving state is `Finishing`, removing set all merged to `None`
/// and spawn a new tile.
pub fn finish_moving(
    mut moving_state: ResMut<MovingState>,
    mut spawn_tile_events: ResMut<Events<SpawnTileEvent>>,
    mut merged: Query<&mut Option<Merged>>,
) {
    if let MovingState::Finishing { moved } = *moving_state {
        // Setting all the merged to `None`.
        for mut merged in merged.iter_mut() {
            if merged.is_some() {
                *merged = None;
            }
        }

        // If some tiles have been moved, spawn a new tile.
        *moving_state = if moved {
            spawn_tile_events.send(SpawnTileEvent::default());
            MovingState::CheckingMoveable
        } else {
            MovingState::Idle
        }
    }
}
