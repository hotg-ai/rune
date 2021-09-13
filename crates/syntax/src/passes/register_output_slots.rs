use indexmap::IndexMap;

use crate::{
    Diagnostics,
    hir::{HirId, NameTable, Node, Slot, Type},
    passes::helpers,
    utils::{Builtins, HirIds},
    yaml::Stage,
};

pub(crate) fn run(
    ids: &mut HirIds,
    pipeline: &IndexMap<String, Stage>,
    types: &mut IndexMap<HirId, Type>,
    builtins: &Builtins,
    names: &NameTable,
    slots: &mut IndexMap<HirId, Slot>,
    stages: &mut IndexMap<HirId, Node>,
    diags: &mut Diagnostics,
) {
    for (name, stage) in pipeline {
        let node_id = match names.get_id(name) {
            Some(id) => id,
            None => continue,
        };

        let mut output_slots = Vec::new();

        for ty in stage.output_types() {
            let element_type =
                helpers::intern_type(ids, ty, types, &builtins, diags);
            let id = ids.next();
            slots.insert(
                id,
                Slot {
                    element_type,
                    input_node: node_id,
                    output_node: HirId::ERROR,
                },
            );
            output_slots.push(id);
        }

        let node = stages.get_mut(&node_id).unwrap();
        node.output_slots = output_slots;
    }
}
