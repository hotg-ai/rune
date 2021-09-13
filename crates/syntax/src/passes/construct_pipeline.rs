use codespan_reporting::diagnostic::Diagnostic;
use indexmap::IndexMap;

use crate::{hir::HirId, passes::Context, yaml::Stage};

pub(crate) fn run(ctx: &mut Context<'_>, pipeline: &IndexMap<String, Stage>) {
    for (name, stage) in pipeline {
        let node_id = match ctx.rune.get_id_by_name(name) {
            Some(id) => id,
            None => continue,
        };

        let mut input_slots = Vec::new();

        for input in stage.inputs() {
            let incoming_node_id = match ctx.rune.get_id_by_name(&input.name) {
                Some(id) => id,
                None => {
                    let diag = Diagnostic::error().with_message(format!(
                        "No node associated with \"{}\"",
                        input
                    ));
                    ctx.diags.push(diag);
                    input_slots.push(HirId::ERROR);
                    continue;
                },
            };

            let incoming_node = ctx.rune.get_stage(&incoming_node_id).unwrap();

            if incoming_node.output_slots.is_empty() {
                let diag = Diagnostic::error().with_message(format!(
                            "The \"{}\" stage tried to connect to \"{}\" but that stage doesn't have any outputs",
                            name,
                            input
                        ));
                ctx.diags.push(diag);
                input_slots.push(HirId::ERROR);
                continue;
            }

            let input_index = input.index.unwrap_or(0);
            match incoming_node.output_slots.get(input_index).copied() {
                Some(slot_id) => {
                    input_slots.push(slot_id);
                    let slot = ctx.rune.slots.get_mut(&slot_id).unwrap();
                    slot.output_node = node_id;
                },
                None => {
                    let diag = Diagnostic::error().with_message(format!(
                            "The \"{}\" stage tried to connect to \"{}\" but that stage only has {} outputs",
                            name,
                            input,
                            incoming_node.output_slots.len(),
                        ));
                    ctx.diags.push(diag);
                    input_slots.push(HirId::ERROR);
                    continue;
                },
            }
        }

        let node = ctx.rune.get_stage_mut(&node_id).unwrap();
        node.input_slots = input_slots;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Diagnostics,
        passes::{construct_pipeline, register_output_slots, register_stages},
        utils::dummy_document,
        yaml::Document,
    };

    use super::*;

    #[test]
    fn construct_the_pipeline() {
        let pipeline = match dummy_document() {
            Document::V1 { pipeline, .. } => pipeline,
        };
        let mut diags = Diagnostics::new();
        let mut ctx = Context::new(&mut diags);
        register_stages::run(&mut ctx, &pipeline);
        register_output_slots::run(&mut ctx, &pipeline);
        run(&mut ctx, &pipeline);
        let edges = vec![
            ("audio", "fft"),
            ("fft", "model"),
            ("model", "label"),
            ("label", "output"),
        ];

        construct_pipeline::run(&mut ctx, &pipeline);

        assert!(ctx.diags.is_empty(), "{:?}", ctx.diags);
        for (from, to) in edges {
            println!("{:?} => {:?}", from, to);
            let from_id = ctx.rune.get_id_by_name(from).unwrap();
            let to_id = ctx.rune.get_id_by_name(to).unwrap();

            assert!(ctx.rune.has_connection(from_id, to_id));
        }
    }
}
