use codespan_reporting::diagnostic::Diagnostic;
use indexmap::IndexMap;

use crate::{
    Diagnostics,
    hir::{HirId, NameTable, Node, Slot},
    yaml::Stage,
};

pub(crate) fn run(
    pipeline: &IndexMap<String, Stage>,
    names: &NameTable,
    stages: &mut IndexMap<HirId, Node>,
    slots: &mut IndexMap<HirId, Slot>,
    diags: &mut Diagnostics,
) {
    for (name, stage) in pipeline {
        let node_id = match names.get_id(name) {
            Some(id) => id,
            None => continue,
        };

        let mut input_slots = Vec::new();

        for input in stage.inputs() {
            let incoming_node_id = match names.get_id(&input.name) {
                Some(id) => id,
                None => {
                    let diag = Diagnostic::error().with_message(format!(
                        "No node associated with \"{}\"",
                        input
                    ));
                    diags.push(diag);
                    input_slots.push(HirId::ERROR);
                    continue;
                },
            };

            let incoming_node = stages.get(&incoming_node_id).unwrap();

            if incoming_node.output_slots.is_empty() {
                let diag = Diagnostic::error().with_message(format!(
                            "The \"{}\" stage tried to connect to \"{}\" but that stage doesn't have any outputs",
                            name,
                            input
                        ));
                diags.push(diag);
                input_slots.push(HirId::ERROR);
                continue;
            }

            let input_index = input.index.unwrap_or(0);
            match incoming_node.output_slots.get(input_index).copied() {
                Some(slot_id) => {
                    input_slots.push(slot_id);
                    let slot = slots.get_mut(&slot_id).unwrap();
                    slot.output_node = node_id;
                },
                None => {
                    let diag = Diagnostic::error().with_message(format!(
                            "The \"{}\" stage tried to connect to \"{}\" but that stage only has {} outputs",
                            name,
                            input,
                            incoming_node.output_slots.len(),
                        ));
                    diags.push(diag);
                    input_slots.push(HirId::ERROR);
                    continue;
                },
            }
        }

        let node = stages.get_mut(&node_id).unwrap();
        node.input_slots = input_slots;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Diagnostics,
        passes::{
            construct_pipeline, helpers, register_output_slots, register_stages,
        },
        utils::{Builtins, HirIds, dummy_document},
        yaml::{Document, DocumentV1},
    };

    use super::*;

    #[test]
    fn construct_the_pipeline() {
        let pipeline = match dummy_document() {
            Document::V1(DocumentV1 { pipeline, .. }) => pipeline,
        };
        let mut diags = Diagnostics::new();
        let mut ids = HirIds::new();
        let builtins = Builtins::new(&mut ids);
        let mut names = NameTable::default();
        let spans = IndexMap::default();
        let mut stages = IndexMap::default();
        let resources = IndexMap::default();
        let mut types = IndexMap::default();
        let mut slots = IndexMap::default();

        register_stages::run(
            &mut ids,
            &pipeline,
            &spans,
            &mut stages,
            &resources,
            &mut names,
            &mut diags,
        );
        register_output_slots::run(
            &mut ids,
            &pipeline,
            &mut types,
            &builtins,
            &names,
            &mut slots,
            &mut stages,
            &mut diags,
        );
        run(&pipeline, &names, &mut stages, &mut slots, &mut diags);
        let edges = vec![
            ("audio", "fft"),
            ("fft", "model"),
            ("model", "label"),
            ("label", "output"),
        ];

        construct_pipeline::run(
            &pipeline,
            &names,
            &mut stages,
            &mut slots,
            &mut diags,
        );

        assert!(diags.is_empty(), "{:?}", diags);
        for (from, to) in edges {
            println!("{:?} => {:?}", from, to);
            let from_id = names.get_id(from).unwrap();
            let to_id = names.get_id(to).unwrap();

            assert!(helpers::has_connection(&stages, from_id, to_id));
        }
    }
}
