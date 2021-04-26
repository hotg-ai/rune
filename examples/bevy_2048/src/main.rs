pub mod audio;
mod board;
mod common;
mod movement;
mod score;
mod tile_spawning;
mod ui;

use bevy::{prelude::*, render::pass::ClearColor};
use common::{GameSizePlugin, GameState, Tile};
use cpal::traits::StreamTrait;
use movement::MovementPlugin;
use score::{Score, ScoreSystemPlugin};
use tile_spawning::{Despawn, SpawnTileEvent, SpawnTilePlugin};
use ui::UiPlugin;

#[macro_use]
extern crate savefile_derive;

/// Number of tiles to spawn at start.
pub const STARTING_TILES: usize = 2;

fn main() {
    let (stream, samples) = audio::start_recording().unwrap();

    stream.play().unwrap();

    App::build()
        // Set window title.
        .add_resource(WindowDescriptor {
            title: "Bevy 2048".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameSizePlugin)
        .add_plugin(SpawnTilePlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(ScoreSystemPlugin)
        .add_plugin(UiPlugin)
        .init_resource::<GameState>()
        // Set background color.
        .add_resource(ClearColor(Color::rgb_u8(250, 248, 239)))
        .add_startup_system(setup.system())
        .add_startup_system(board::spawn_board.system())
        .add_system(new_game.system())
        .add_system(space_new_game.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut spawn_tile_events: ResMut<Events<SpawnTileEvent>>,
) {
    // Camera.
    commands.spawn(Camera2dComponents::default());

    // Spawning tiles at the beginning.
    spawn_tile_events.send(SpawnTileEvent {
        count: STARTING_TILES,
    });
}

fn new_game(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut spawn_tile_events: ResMut<Events<SpawnTileEvent>>,
    mut score: ResMut<Score>,
    tiles: Query<With<Tile, Entity>>,
) {
    if matches!(*game_state, GameState::Restarting) {
        for entity in &mut tiles.iter() {
            commands.insert_one(entity, Despawn);
        }

        spawn_tile_events.send(SpawnTileEvent {
            count: STARTING_TILES,
        });

        score.0 = 0;
        *game_state = GameState::Play;
    }
}

fn space_new_game(
    mut game_state: ResMut<GameState>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        *game_state = GameState::Restarting;
    }
}
