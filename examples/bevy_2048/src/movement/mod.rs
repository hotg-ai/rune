//! This module contains the implementation of the systems and components in
//! order to move the tiles.

mod check_moveable;
mod finish_moving;
mod merge_animation;
mod merging;
mod moving_animation;
mod moving_direction;
mod moving_input;
mod moving_state;
mod set_moving;

pub use merge_animation::MergeAnimation;
pub use moving_animation::MovingAnimation;
pub use moving_direction::MovingDirection;
pub use moving_state::MovingState;
/// Component to tell if a tile is moving or not.
pub struct Moving;

/// Component to tell if a tile has been merged or not.
pub struct Merged;

use bevy::prelude::*;
use rune_runtime::Capability;
use rune_wasmer_runtime::Runtime;
use runicos_base::BaseImage;
use std::sync::{Arc, RwLock};
use runic_types::Value;
use crate::audio::Samples;

pub struct MovementPlugin {
    samples: Arc<RwLock<Samples>>,
    rune: Vec<u8>,
}

impl MovementPlugin {
    pub fn new(samples: Arc<RwLock<Samples>>, rune_wasm: Vec<u8>) -> Self {
        MovementPlugin {
            samples,
            rune: rune_wasm,
        }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        let mut image = BaseImage::default();
        let samples = Arc::clone(&self.samples);
        image.with_sound(move || {
            Ok(Box::new(Microphone::new(Arc::clone(&samples))))
        });
        let runtime = Runtime::load(&self.rune, image).unwrap();

        app.init_resource::<MovingAnimation>()
            .init_resource::<MovingState>()
            .init_resource::<Option<MovingDirection>>()
            .add_resource(MovingDirection::Left)
            .add_resource(runtime)
            .add_system(execute_rune.system())
            .add_system(moving_input::moving_input.system())
            .add_system(moving_input::next_direction.system())
            .add_system(set_moving::set_moving.system())
            .add_system(moving_animation::moving_animation.system())
            .add_system(merging::merging.system())
            .add_system(merge_animation::merge_animation.system())
            .add_system(finish_moving::finish_moving.system())
            // This system should run after the new tile have spawned.
            .add_system_to_stage(
                crate::tile_spawning::POST_SPAWN_STAGE,
                check_moveable::check_moveable.system(),
            );
    }
}

fn execute_rune(mut runtime: ResMut<Runtime>) {
    log::debug!("Executing the Rune");
    runtime.call().unwrap();
}

#[derive(Debug, Clone)]
struct Microphone {
    samples: Arc<RwLock<Samples>>,
}

impl Microphone {
    pub fn new(samples: Arc<RwLock<Samples>>) -> Self { Microphone { samples } }
}

impl Capability for Microphone {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, anyhow::Error> {
        let samples = self.samples.read().unwrap();

        let mut bytes_written = 0;

        for (chunk, sample) in buffer
            .chunks_mut(std::mem::size_of::<i16>())
            .zip(samples.iter())
        {
            let bytes = sample.to_le_bytes();
            chunk.copy_from_slice(&bytes);

            bytes_written += bytes.len();
        }

        Ok(bytes_written)
    }

    fn set_parameter(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), rune_runtime::ParameterError> {
        println!("Setting {} = {}", name, value);

        Ok(())
    }
}
