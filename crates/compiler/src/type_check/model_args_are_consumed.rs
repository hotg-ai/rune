use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::{world::SubWorld, Query};

use crate::{
    lowering::{Model, Name},
    Diagnostics,
};

/// Check that all model arguments were consumed during the lowering process,
/// emitting a warning for any that weren't.
#[legion::system]
pub(crate) fn run(
    world: &SubWorld,
    #[resource] diags: &mut Diagnostics,
    models: &mut Query<(&Name, &Span, &Model)>,
) {
    models.for_each(world, |(n, s, m)| {
        if !m.args.is_empty() {
            let unused_args: Vec<_> =
                m.args.keys().map(|s| s.as_str()).collect();
            diags.push(unused_model_arguments_diagnostic(n, *s, &unused_args));
        }
    });
}

fn unused_model_arguments_diagnostic(
    name: &Name,
    span: Span,
    unused_args: &[&str],
) -> Diagnostic<()> {
    Diagnostic::warning()
        .with_message(format!(
            "Unused arguments for {}: {}",
            name,
            unused_args.join(", ")
        ))
        .with_labels(vec![Label::primary((), span)])
}
