//! This module cotains the implementation of the SpawnTile event, reader and system.

use crate::common::{GameSize, Position, Tile};
use crate::movement::{Merged, Moving};
use bevy::prelude::*;
use rand::Rng;

use super::SpawnAnimation;

/// Event for spawning new tiles.
pub struct SpawnTileEvent {
    pub count: usize,
}

impl Default for SpawnTileEvent {
    /// Spawns 1 tile.
    fn default() -> Self {
        Self { count: 1 }
    }
}

/// Event listener for SpawnTileEvent.
#[derive(Default)]
pub struct SpawnTileListener {
    pub reader: EventReader<SpawnTileEvent>,
}

/// Spawning a new tile for every SpawnTileEvent event.
pub fn spawn_tiles(
    mut commands: Commands,
    game_size: Res<GameSize>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut listener: ResMut<SpawnTileListener>,
    spawn_events: Res<Events<SpawnTileEvent>>,
    positions: Query<With<Tile, &Position>>,
) {
    // Vector of empty tiles for all the iterations.
    let mut free_pos = None;
    for ev in listener.reader.iter(&spawn_events) {
        for _ in 0..ev.count {
            if free_pos.is_none() {
                // Creating vector of empty tiles.
                let mut vec = Vec::new();
                for row in 0..4 {
                    for col in 0..4 {
                        vec.push(Position { row, col });
                    }
                }

                // Removing the existing tiles from the vector.
                for pos in &mut positions.iter() {
                    if let Some(idx) = vec.iter().position(|x| *x == *pos) {
                        vec.remove(idx);
                    }
                }

                free_pos = Some(vec);
            }

            let vec = free_pos.as_mut().unwrap();

            // Checking that the board is not full.
            if vec.len() != 0 {
                // Choosing a random empty tile.
                let mut rng = rand::thread_rng();
                let idx = rng.gen_range(0, vec.len());
                let pos = vec.remove(idx);

                // Choosing the new tile's level.
                let tile = Tile {
                    level: if rng.gen_bool(0.8) { 0 } else { 1 },
                };

                // Spawning the new tile.
                commands
                    .spawn(SpriteComponents {
                        material: materials.add(tile.color().into()),
                        transform: Transform::from_translation(pos.to_vec3(*game_size)),
                        ..Default::default()
                    })
                    .with(tile)
                    .with(pos)
                    .with(SpawnAnimation::default())
                    .with(Option::<Moving>::None)
                    .with(Option::<Merged>::None);
            } else {
                #[cfg(debug_assertions)]
                panic!("spawn_tiles(): Tried to spawn a tile when the board was full.")
            }
        }
    }
}
