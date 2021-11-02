//! The lowering phase.

mod components;
mod load_model_data;
mod load_resource_data;
mod register_names;
mod register_resources;
mod register_stages;
mod register_tensors;
mod update_nametable;

pub use components::*;
use legion::Registry;

use crate::{phases::Phase, serialize::RegistryExt};

pub fn phase() -> Phase {
    Phase::with_setup(|res| {
        res.insert(NameTable::default());
    })
    .and_then(register_names::run_system)
    .and_then(update_nametable::run_system)
    .and_then(register_resources::run_system)
    .and_then(register_stages::run_system)
    .and_then(register_tensors::run_system)
    .and_then(load_resource_data::run_system)
    .and_then(load_model_data::run_system)
}

pub(crate) fn register_components(registry: &mut Registry<String>) {
    registry
        .register_with_type_name::<Inputs>()
        .register_with_type_name::<Model>()
        .register_with_type_name::<ModelFile>()
        .register_with_type_name::<Name>()
        .register_with_type_name::<NameTable>()
        .register_with_type_name::<Outputs>()
        .register_with_type_name::<PipelineNode>()
        .register_with_type_name::<ProcBlock>()
        .register_with_type_name::<Resource>()
        .register_with_type_name::<ResourceSource>()
        .register_with_type_name::<Sink>()
        .register_with_type_name::<SinkKind>()
        .register_with_type_name::<Source>()
        .register_with_type_name::<SourceKind>()
        .register_with_type_name::<Tensor>()
        .register_with_type_name::<ResourceData>()
        .register_with_type_name::<Mimetype>()
        .register_with_type_name::<ModelData>();
}
