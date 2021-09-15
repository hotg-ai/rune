//! The lowering phase.

mod register_names;
mod register_resources;
mod register_stages;
mod register_tensors;
mod update_nametable;

use crate::phases::Phase;

pub fn phase() -> Phase {
    Phase::new()
        .and_then(register_names::run_system())
        .and_then(update_nametable::run_system())
        .and_then(register_resources::run_system())
        .and_then(register_stages::run_system())
        .and_then(register_tensors::run_system())
}
