//! This module contians the implementation of the CheckingMoveable state's system.

use bevy::prelude::*;

use crate::common::{GameState, Position, Tile};

use super::MovingState;

/// When the moving state is `CheckingMoveable`, checking if it is a gameover
/// by looking if there are tiles that can move.
pub fn check_moveable(
    mut game_state: ResMut<GameState>,
    mut moving_state: ResMut<MovingState>,
    tiles: Query<(&Tile, &Position)>,
) {
    if matches!(*game_state, GameState::Play) {
        if matches!(*moving_state, MovingState::CheckingMoveable) {
            // Checking if the board is full.
            let len = tiles.iter().len();
            if len == 16 {
                // Creating a 1d array board.
                let mut iter = tiles.iter();
                let null_tile = Tile { level: 0 };
                let mut board = [&null_tile; 16];
                for (tile, position) in &mut iter {
                    board[position.index()] = tile;
                }

                // Checking if there are some neighbor tiles with the same level.
                let mut gameover = true;
                for row in 0..4 {
                    for col in 0..4 {
                        let pos = Position { row, col };

                        if row < 3 {
                            let up = Position { row: row + 1, col };
                            if board[pos.index()].level == board[up.index()].level {
                                gameover = false;
                                break;
                            }
                        }

                        if col < 3 {
                            let right = Position { row, col: col + 1 };
                            if board[pos.index()].level == board[right.index()].level {
                                gameover = false;
                                break;
                            }
                        }
                    }
                }

                if gameover {
                    *game_state = GameState::GameOver;
                }
            }

            *moving_state = MovingState::Idle;
        }
    }
}
