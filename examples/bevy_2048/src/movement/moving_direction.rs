//! This module contains the implementation of the MovingDirection component.
use crate::common::Position;
use bevy::prelude::*;
use std::convert::TryFrom;

/// The direction of the movement.
/// This is a global resource because all tiles
/// moving to the same direction.
#[derive(Debug, Copy, Clone)]
pub enum MovingDirection {
    Left,
    Up,
    Right,
    Down,
}

impl MovingDirection {
    /// Returns the new position after the movement according
    /// to the direction.
    /// Returns `None` if the new position is out of bounds.
    pub fn moved_position(&self, position: &Position) -> Option<Position> {
        match self {
            Self::Left if position.col > 0 => Some(Position {
                row: position.row,
                col: position.col - 1,
            }),
            Self::Up if position.row < 3 => Some(Position {
                row: position.row + 1,
                col: position.col,
            }),
            Self::Right if position.col < 3 => Some(Position {
                row: position.row,
                col: position.col + 1,
            }),
            Self::Down if position.row > 0 => Some(Position {
                row: position.row - 1,
                col: position.col,
            }),
            // If the new position is out of bounds.
            _ => None,
        }
    }

    /// Returns an array sorted by the order of tiles should
    /// be iterated when checking which tile should move.
    pub fn board_iteration(&self) -> [Position; 16] {
        let mut result: [Position; 16] = Default::default();
        let mut index = 0;

        // When moving to the left, secondary is the rows
        // because it doesn't matter which row should
        // be checked first.
        for secondary in 0..4 {
            // When moving to the left, primary is the columns
            // because the order of checking does matter.
            for mut primary in 0..4 {
                // Reversing primary.
                if let Self::Up | Self::Right = self {
                    primary = 3 - primary;
                }

                // Saving the position in the array.
                result[index] = match self {
                    Self::Left | Self::Right => Position {
                        row: secondary,
                        col: primary,
                    },
                    Self::Up | Self::Down => Position {
                        row: primary,
                        col: secondary,
                    },
                };

                index += 1;
            }
        }

        result
    }
}

impl TryFrom<&KeyCode> for MovingDirection {
    type Error = &'static str;

    /// Converts the arrows and a,w,d,s keys into a direction.
    fn try_from(key: &KeyCode) -> Result<Self, Self::Error> {
        match key {
            KeyCode::Left | KeyCode::A => Ok(Self::Left),
            KeyCode::Up | KeyCode::W => Ok(Self::Up),
            KeyCode::Right | KeyCode::D => Ok(Self::Right),
            KeyCode::Down | KeyCode::S => Ok(Self::Down),
            _ => Err("Couldn't convert the key into a direction"),
        }
    }
}

impl From<MovingDirection> for Vec3 {
    /// Converts a direction into a normalized vec3.
    fn from(direction: MovingDirection) -> Self {
        match direction {
            MovingDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            MovingDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            MovingDirection::Right => Vec3::new(1.0, 0.0, 0.0),
            MovingDirection::Down => Vec3::new(0.0, -1.0, 0.0),
        }
    }
}
