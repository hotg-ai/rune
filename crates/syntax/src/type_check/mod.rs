//! The type checking phase.

mod components;
mod load_model_data;
mod load_resource_data;

pub use components::*;

use legion::Registry;
use crate::{phases::Phase, serialize::RegistryExt};

pub fn phase() -> Phase {
    Phase::new()
        .and_then(load_resource_data::run_system)
        .and_then(load_model_data::run_system)
}

pub(crate) fn register_components(registry: &mut Registry<String>) {
    registry
        .register_with_type_name::<ResourceData>()
        .register_with_type_name::<ModelData>();
}
