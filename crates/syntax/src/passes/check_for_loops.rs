use std::collections::{HashSet, VecDeque};

use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    hir::{HirId, Rune},
    passes::Context,
};

pub(crate) fn run(ctx: &mut Context<'_>) {
    if let Some(cycle) = next_cycle(ctx) {
        let (first, middle) = match cycle.as_slice() {
            [first, middle @ ..] => (first, middle),
            _ => unreachable!("A cycle must have at least 2 items"),
        };

        let mut diag = Diagnostic::error().with_message(format!(
            "Cycle detected when checking \"{}\"",
            ctx.rune.get_name_by_id(*first).unwrap()
        ));

        if let Some(span) = ctx.rune.spans.get(first) {
            diag = diag.with_labels(vec![Label::primary((), *span)]);
        }

        let mut notes = Vec::new();

        for middle_id in middle {
            let msg = format!(
                "... which receives input from \"{}\"...",
                ctx.rune.get_name_by_id(*middle_id).unwrap()
            );
            notes.push(msg);
        }

        let closing_message = format!(
            "... which receives input from \"{}\", completing the cycle.",
            ctx.rune.get_name_by_id(*first).unwrap()
        );
        notes.push(closing_message);

        ctx.diags.push(diag.with_notes(notes));
    }
}

fn next_cycle(ctx: &mut Context<'_>) -> Option<Vec<HirId>> {
    // https://www.geeksforgeeks.org/detect-cycle-in-a-graph/
    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();

    for id in ctx.rune.stages().map(|(id, _)| id) {
        if detect_cycles(id, &ctx.rune, &mut visited, &mut stack) {
            return Some(stack.into());
        }
    }

    None
}

fn detect_cycles(
    id: HirId,
    rune: &Rune,
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

    let incoming_nodes = rune
        .get_stage(&id)
        .unwrap()
        .input_slots
        .iter()
        .map(|slot_id| rune.slots[slot_id].input_node);

    for incoming_node in incoming_nodes {
        if detect_cycles(incoming_node, rune, visited, stack) {
            return true;
        }
    }

    let got = stack.pop_back();
    debug_assert_eq!(got, Some(id));

    false
}
