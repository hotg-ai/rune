use std::collections::HashMap;

use petgraph::{
    EdgeDirection,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};

use crate::{
    Diagnostics,
    hir::{Edge, HirId, Stage, Type},
};

pub(crate) fn infer<FileId: Copy>(
    graph: &mut DiGraph<Stage, Edge>,
    input_types: &HashMap<NodeIndex, HirId>,
    output_types: &HashMap<NodeIndex, HirId>,
    _types: &HashMap<HirId, Type>,
    _file_id: FileId,
    _diags: &mut Diagnostics<FileId>,
) {
    // populate the edges we *do* know
    let known_inputs = input_types.iter().flat_map(|(&node_ix, &type_id)| {
        graph
            .edges_directed(node_ix, EdgeDirection::Incoming)
            .map(move |edge| (edge.id(), type_id))
    });
    let known_outputs = output_types.iter().flat_map(|(&node_ix, &type_id)| {
        graph
            .edges_directed(node_ix, EdgeDirection::Outgoing)
            .map(move |edge| (edge.id(), type_id))
    });
    let known: Vec<_> = known_inputs.chain(known_outputs).collect();

    // TODO: Check for duplicates
    for (edge_ix, type_id) in known {
        let edge = graph.edge_weight_mut(edge_ix).unwrap();
        edge.type_id = type_id;
    }
}
