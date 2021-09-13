use std::{collections::HashMap, ops::Deref};
use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::{Entity, Query, world::SubWorld};
use crate::{Diagnostics, hir::Name};

/// Update the [`NameTable`] resource so we can track all the named items in a
/// Runefile.
#[legion::system]
pub(crate) fn run(
    world: &mut SubWorld,
    #[resource] diags: &mut Diagnostics,
    #[resource] names: &mut NameTable,
    named_items: &mut Query<(Entity, &Name, &Span)>,
) {
    names.0.clear();

    let mut lookup_table: HashMap<&Name, Vec<_>> = HashMap::new();

    named_items.for_each(world, |(e, n, s)| {
        let items = lookup_table.entry(n).or_default();
        items.push((e, s));
        // Note: Keep them sorted by location in the source file so the
        // "first definition was here" message points at the item closest to the
        // top.
        items.sort_by_key(|(_, &s)| s);
    });

    for (name, items) in lookup_table {
        match items.as_slice() {
            [] => unreachable!(),
            [(&ent, _)] => {
                // The happy path - the file had just one item with this name.
                names.0.insert(name.clone(), ent);
            },
            [(&ent, &first_definition), others @ ..] => {
                // emit an error message and only remember the first
                let diag = duplicate_name_diagnostic(
                    name,
                    first_definition,
                    others.iter().map(|(_, &s)| s),
                );
                diags.push(diag);

                names.0.insert(name.clone(), ent);
            },
        }
    }
}

fn duplicate_name_diagnostic(
    name: &Name,
    first_definition: Span,
    duplicates: impl Iterator<Item = Span>,
) -> Diagnostic<()> {
    let primary = Label::primary((), first_definition)
        .with_message("The first definition is here");

    let mut labels = vec![primary];

    for duplicate in duplicates {
        labels.push(
            Label::secondary((), duplicate).with_message("Redefined here"),
        );
    }

    Diagnostic::error()
        .with_message(format!(
            "The name \"{}\" is defined multiple times",
            name
        ))
        .with_labels(labels)
}

/// A lookup table mapping [`Name`] components back to their [`Entity`].
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NameTable(HashMap<Name, Entity>);

impl Deref for NameTable {
    type Target = HashMap<Name, Entity>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
