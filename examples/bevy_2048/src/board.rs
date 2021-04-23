use bevy::prelude::*;

use crate::common::{GameSize, Position};

/// An identifier for the board background's entity.
pub struct Board;

/// An identifier for the empty-tiles' entities.
pub struct EmptyTile;

/// This system spawns the board and the emtpy-tiles.
pub fn spawn_board(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_size: Res<GameSize>,
) {
    // Board background.
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb_u8(119, 110, 101).into()),
            sprite: Sprite::new(Vec2::new(game_size.board_size(), game_size.board_size())),
            ..Default::default()
        })
        .with(Board);

    // Creating a grid of empty tiles.
    for row in 0..4 {
        for col in 0..4 {
            let position = Position { row, col };

            commands
                .spawn(SpriteComponents {
                    material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
                    sprite: Sprite::new(Vec2::new(game_size.tile_size(), game_size.tile_size())),
                    transform: Transform::from_translation(position.to_vec3(*game_size)),
                    ..Default::default()
                })
                .with(position)
                .with(EmptyTile);
        }
    }
}
