use std::collections::{HashMap, HashSet, VecDeque};

use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::{Entity, Query, world::SubWorld};

use crate::{
    Diagnostics,
    lowering::{Name, Outputs},
};

#[legion::system]
pub(crate) fn run(
    world: &SubWorld,
    #[resource] diags: &mut Diagnostics,
    names: &mut Query<(&Name, &Span)>,
    query: &mut Query<(Entity, &Outputs)>,
) {
    // construct an adjacency graph where edges go from a node to its output.
    let mut outputs = HashMap::new();
    query.for_each(world, |(&ent, out)| {
        outputs.insert(ent, out.tensors.as_slice());
    });

    if let Some(cycle) = next_cycle(&outputs) {
        let cycle: Vec<_> = cycle
            .iter()
            .filter_map(|&ent| names.get(world, ent).ok())
            .collect();

        let diag = cycle_detected_diagnostic(&cycle);
        diags.push(diag);
    }
}

fn cycle_detected_diagnostic(
    cycle: &[(&Name, &Span)],
) -> codespan_reporting::diagnostic::Diagnostic<()> {
    let ((name, span), middle) = match cycle {
        [first, middle @ ..] => (*first, middle),
        _ => unreachable!("A cycle must have at least 2 items"),
    };

    let mut diag = Diagnostic::error()
        .with_message(format!("Cycle detected when checking \"{}\"", name));

    diag = diag.with_labels(vec![Label::primary((), *span)]);

    let mut notes = Vec::new();

    for (name, _) in middle {
        let msg = format!("... which passes data to \"{}\"...", name);
        notes.push(msg);
    }

    let closing_message = format!(
        "... which passes data to \"{}\", completing the cycle.",
        name
    );
    notes.push(closing_message);

    diag.with_notes(notes)
}

fn next_cycle(outputs: &HashMap<Entity, &[Entity]>) -> Option<Vec<Entity>> {
    // https://www.geeksforgeeks.org/detect-cycle-in-a-graph/
    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();

    for ent in outputs.keys().copied() {
        if detect_cycles(ent, outputs, &mut visited, &mut stack) {
            return Some(stack.into());
        }
    }

    None
}

fn detect_cycles(
    ent: Entity,
    outputs: &HashMap<Entity, &[Entity]>,
    visited: &mut HashSet<Entity>,
    stack: &mut VecDeque<Entity>,
) -> bool {
    if stack.contains(&ent) {
        // We've detected a cycle, remove everything before our id so the stack
        // is left just containing the cycle
        while stack.front() != Some(&ent) {
            stack.pop_front();
        }

        return true;
    } else if visited.contains(&ent) {
        return false;
    }

    visited.insert(ent);
    stack.push_back(ent);

    let outgoing_node = outputs.get(&ent).copied().unwrap_or_default();

    for &outgoing_node in outgoing_node {
        if detect_cycles(outgoing_node, outputs, visited, stack) {
            return true;
        }
    }

    let got = stack.pop_back();
    debug_assert_eq!(got, Some(ent));

    false
}
