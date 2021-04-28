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

use anyhow::{Context, Error};
pub use merge_animation::MergeAnimation;
pub use moving_animation::MovingAnimation;
pub use moving_direction::MovingDirection;
pub use moving_state::MovingState;
/// Component to tell if a tile is moving or not.
pub struct Moving;

/// Component to tell if a tile has been merged or not.
pub struct Merged;

use std::convert::TryFrom;
use bevy::prelude::*;
use rune_runtime::{Capability, Output, ParameterError};
use rune_wasmer_runtime::Runtime;
use runicos_base::BaseImage;
use std::{
    borrow::Cow,
    sync::{Arc, RwLock},
};
use runic_types::Value;
use crate::audio::Samples;

pub struct MovementPlugin {
    runtime: Runtime,
    current_movement: Arc<RwLock<Option<MovingDirection>>>,
}

impl MovementPlugin {
    pub fn load(
        samples: Arc<RwLock<Samples>>,
        rune_wasm: &[u8],
    ) -> Result<Self, Error> {
        let current_movement = Arc::new(RwLock::new(None));
        let current_movement_2 = Arc::clone(&current_movement);

        let mut image = BaseImage::default();
        let samples = Arc::clone(&samples);
        image
            .with_sound(move || {
                Ok(Box::new(Microphone::new(Arc::clone(&samples))))
            })
            .with_serial(move || {
                let current_movement = Arc::clone(&current_movement_2);
                Ok(Box::new(Serial::new(current_movement)))
            });
        let runtime = Runtime::load(rune_wasm, image)?;

        Ok(MovementPlugin {
            runtime,
            current_movement,
        })
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        let MovementPlugin {
            runtime,
            current_movement,
        } = self;

        app.init_resource::<MovingAnimation>()
            .init_resource::<MovingState>()
            .init_resource::<Option<MovingDirection>>()
            .add_resource(MovingDirection::Left)
            .add_resource(runtime.clone())
            .add_resource(Arc::clone(current_movement))
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
    sample_rate: i32,
    sample_duration_ms: i32,
}

impl Microphone {
    pub fn new(samples: Arc<RwLock<Samples>>) -> Self {
        Microphone {
            samples,
            sample_rate: 0,
            sample_duration_ms: 0,
        }
    }

    fn update_buffer_capacity(&self) {
        let mut samples = self.samples.write().unwrap();
        let capacity = self.sample_rate * self.sample_duration_ms / 1000;
        samples.set_capacity(capacity as usize);
    }
}

impl Capability for Microphone {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, anyhow::Error> {
        let samples = self.samples.read().unwrap();

        let mut bytes_written = 0;

        for (chunk, sample) in buffer
            .chunks_mut(std::mem::size_of::<f32>())
            .zip(samples.iter())
        {
            let bytes = sample.to_le_bytes();
            chunk.copy_from_slice(&bytes);

            bytes_written += bytes.len();
        }

        log::debug!("Wrote {} bytes", bytes_written);

        Ok(bytes_written)
    }

    fn set_parameter(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), ParameterError> {
        log::info!("Setting {} = {}", name, value);

        match name {
            "hz" => {
                self.sample_rate = i32::try_from(value)?;
                log::info!("Sample Rate = {} hz", self.sample_rate);
                self.update_buffer_capacity();
            },
            "sample_duration_ms" => {
                self.sample_duration_ms = i32::try_from(value)?;
                log::info!("Sample Duration = {} ms", self.sample_duration_ms);
                self.update_buffer_capacity();
            },
            _ => {
                log::info!("Setting unknown property \"{}\" = {}", name, value);
            },
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Serial {
    current_movement: Arc<RwLock<Option<MovingDirection>>>,
}

impl Serial {
    fn new(current_movement: Arc<RwLock<Option<MovingDirection>>>) -> Self {
        Serial { current_movement }
    }
}

impl Output for Serial {
    fn consume(&mut self, buffer: &[u8]) -> Result<(), anyhow::Error> {
        let msg: Message = serde_json::from_slice(buffer)
            .context("Unable to deserialize the message")?;

        let mut current_movement = self.current_movement.write().unwrap();

        *current_movement = match &*msg.string {
            "unknown" | "silence" => None,
            "up" => Some(MovingDirection::Up),
            "down" => Some(MovingDirection::Down),
            "right" => Some(MovingDirection::Right),
            "left" => Some(MovingDirection::Left),
            other => anyhow::bail!("Unknown label: \"{}\"", other),
        };

        log::info!("{:?} => {:?}", msg, *current_movement);

        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
// {"type_name":"&st\nr","channel":2,"string":"unknown"}
struct Message<'a> {
    type_name: Cow<'a, str>,
    channel: usize,
    string: Cow<'a, str>,
}
