use indexmap::IndexMap;

use crate::{hir::{HirId, Slot}, passes::Context, yaml::Stage};

pub(crate) fn run(ctx: &mut Context<'_>, pipeline: &IndexMap<String, Stage>) {
    for (name, stage) in pipeline {
        let node_id = match ctx.rune.get_id_by_name(name) {
            Some(id) => id,
            None => continue,
        };

        let mut output_slots = Vec::new();

        for ty in stage.output_types() {
            let element_type = ctx.intern_type(ty);
            let id = ctx.ids.next();
            ctx.rune.slots.insert(
                id,
                Slot {
                    element_type,
                    input_node: node_id,
                    output_node: HirId::ERROR,
                },
            );
            output_slots.push(id);
        }

        let node = ctx.rune.get_stage_mut(&node_id).unwrap();
        node.output_slots = output_slots;
    }
}
