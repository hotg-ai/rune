use legion::{
    Entity, Query,
    serialize::{Canon, CustomEntitySerializer},
    systems::CommandBuffer,
    world::SubWorld,
};
use crate::{
    BuildContext,
    codegen::{
        ModelSummary, OutputSummary, ProcBlockSummary, RuneGraph, TensorId,
    },
    lowering::{
        Inputs, Model, ModelFile, Name, Outputs, ProcBlock, Resource, Sink,
        Source, Tensor,
    },
    parse::{ResourceName, ResourceOrString},
};

use super::{CapabilitySummary, RuneSummary};

/// Generate an abbreviated [`RuneGraph`].
#[legion::system]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    #[resource] ctx: &BuildContext,
    capabilities: &mut Query<(&Name, &Source, &Outputs)>,
    tensors: &mut Query<(Entity, &Tensor)>,
    models: &mut Query<(&Name, &Model, &Inputs, &Outputs)>,
    proc_blocks: &mut Query<(&Name, &ProcBlock, &Inputs, &Outputs)>,
    outputs: &mut Query<(&Name, &Sink, &Inputs)>,
    resources: &mut Query<(&Name, &Resource)>,
) {
    let canon = Canon::default();

    let graph = RuneGraph {
        rune: rune_summary(ctx),
        capabilities: capabilities
            .iter(world)
            .map(|(n, s, o)| capability_summary(n, s, o, &canon))
            .collect(),
        models: models
            .iter(world)
            .map(|(n, m, i, o)| {
                model_summary(
                    n,
                    m,
                    i,
                    o,
                    |ent| {
                        resources
                            .get(world, ent)
                            .map(|(name, _)| ResourceName(name.to_string()))
                            .unwrap()
                    },
                    &canon,
                )
            })
            .collect(),
        proc_blocks: proc_blocks
            .iter(world)
            .map(|(n, p, i, o)| proc_block_summary(n, p, i, o, &canon))
            .collect(),
        outputs: outputs
            .iter(world)
            .map(|(n, s, i)| output_summary(n, s, i, &canon))
            .collect(),
        resources: resources
            .iter(world)
            .map(|(name, res)| (name.clone(), res.clone()))
            .collect(),
        tensors: tensors
            .iter(world)
            .map(|(ent, t)| {
                (canon.to_serialized(*ent).to_string().into(), t.0.clone())
            })
            .collect(),
    };

    let graph_section = graph
        .as_custom_section()
        .expect("We should always be able to serialize to JSON");
    cmd.push((graph, graph_section));
}

fn rune_summary(ctx: &BuildContext) -> RuneSummary {
    RuneSummary {
        name: ctx.name.clone(),
    }
}

fn tensor_shapes(tensors: &[Entity], get_tensor: &Canon) -> Vec<TensorId> {
    tensors
        .iter()
        .map(|&ent| get_tensor.to_serialized(ent))
        .map(|t| TensorId(t.to_string()))
        .collect()
}

fn capability_summary(
    name: &Name,
    source: &Source,
    outputs: &Outputs,
    get_tensor: &Canon,
) -> (Name, CapabilitySummary) {
    let summary = CapabilitySummary {
        kind: source.kind.clone(),
        args: source.parameters.clone(),
        outputs: tensor_shapes(&outputs.tensors, get_tensor),
    };

    (name.clone(), summary)
}

fn model_summary(
    name: &Name,
    model: &Model,
    inputs: &Inputs,
    outputs: &Outputs,
    mut resources: impl FnMut(Entity) -> ResourceName,
    get_tensor: &Canon,
) -> (Name, ModelSummary) {
    let file = match &model.model_file {
        ModelFile::FromDisk(path) => {
            ResourceOrString::String(path.display().to_string())
        },
        ModelFile::Resource(entity) => {
            ResourceOrString::Resource(resources(*entity))
        },
    };

    let summary = ModelSummary {
        file,
        inputs: tensor_shapes(&inputs.tensors, get_tensor),
        outputs: tensor_shapes(&outputs.tensors, get_tensor),
    };

    (name.clone(), summary)
}

fn proc_block_summary(
    name: &Name,
    proc_block: &ProcBlock,
    inputs: &Inputs,
    outputs: &Outputs,
    get_tensor: &Canon,
) -> (Name, ProcBlockSummary) {
    let summary = ProcBlockSummary {
        path: proc_block.path.clone(),
        args: proc_block.parameters.clone(),
        inputs: tensor_shapes(&inputs.tensors, get_tensor),
        outputs: tensor_shapes(&outputs.tensors, get_tensor),
    };

    (name.clone(), summary)
}

fn output_summary(
    name: &Name,
    sink: &Sink,
    inputs: &Inputs,
    get_tensor: &Canon,
) -> (Name, OutputSummary) {
    let summary = OutputSummary {
        kind: sink.kind.clone(),
        inputs: tensor_shapes(&inputs.tensors, get_tensor),
    };

    (name.clone(), summary)
}
