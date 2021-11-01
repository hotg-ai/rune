//! The type checking phase.

mod check_for_loops;
mod components;
mod model_args_are_consumed;

pub use components::*;

use legion::Registry;
use crate::phases::Phase;

pub fn phase() -> Phase {
    Phase::new()
        .and_then(check_for_loops::run_system)
        .and_then(model_args_are_consumed::run_system)
}

pub(crate) fn register_components(_registry: &mut Registry<String>) {}
