use std::collections::{HashSet, VecDeque};

use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use indexmap::IndexMap;

use crate::{
    Diagnostics,
    hir::{HirId, NameTable, Node, Slot},
};

pub(crate) fn run(
    stages: &IndexMap<HirId, Node>,
    slots: &IndexMap<HirId, Slot>,
    names: &NameTable,
    spans: &IndexMap<HirId, Span>,
    diags: &mut Diagnostics,
) {
    if let Some(cycle) = next_cycle(stages, slots) {
        let (first, middle) = match cycle.as_slice() {
            [first, middle @ ..] => (first, middle),
            _ => unreachable!("A cycle must have at least 2 items"),
        };

        let mut diag = Diagnostic::error().with_message(format!(
            "Cycle detected when checking \"{}\"",
            names.get_name(*first).unwrap()
        ));

        if let Some(span) = spans.get(first) {
            diag = diag.with_labels(vec![Label::primary((), *span)]);
        }

        let mut notes = Vec::new();

        for middle_id in middle {
            let msg = format!(
                "... which receives input from \"{}\"...",
                names.get_name(*middle_id).unwrap()
            );
            notes.push(msg);
        }

        let closing_message = format!(
            "... which receives input from \"{}\", completing the cycle.",
            names.get_name(*first).unwrap()
        );
        notes.push(closing_message);

        diags.push(diag.with_notes(notes));
    }
}

fn next_cycle(
    stages: &IndexMap<HirId, Node>,
    slots: &IndexMap<HirId, Slot>,
) -> Option<Vec<HirId>> {
    // https://www.geeksforgeeks.org/detect-cycle-in-a-graph/
    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();

    for id in stages.iter().map(|(&id, _)| id) {
        if detect_cycles(id, stages, slots, &mut visited, &mut stack) {
            return Some(stack.into());
        }
    }

    None
}

fn detect_cycles(
    id: HirId,
    stages: &IndexMap<HirId, Node>,
    slots: &IndexMap<HirId, Slot>,
    visited: &mut HashSet<HirId>,
    stack: &mut VecDeque<HirId>,
) -> bool {
    if stack.contains(&id) {
        // We've detected a cycle, remove everything before our id so the stack
        // is left just containing the cycle
        while stack.front() != Some(&id) {
            stack.pop_front();
        }

        return true;
    } else if visited.contains(&id) {
        return false;
    }

    visited.insert(id);
    stack.push_back(id);

    let incoming_nodes = stages
        .get(&id)
        .unwrap()
        .input_slots
        .iter()
        .map(|slot_id| slots.get(slot_id).unwrap().input_node);

    for incoming_node in incoming_nodes {
        if detect_cycles(incoming_node, stages, slots, visited, stack) {
            return true;
        }
    }

    let got = stack.pop_back();
    debug_assert_eq!(got, Some(id));

    false
}
