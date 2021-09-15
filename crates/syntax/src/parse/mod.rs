//! The parsing phase.

mod yaml;

pub use self::yaml::*;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::systems::CommandBuffer;
use crate::{BuildContext, Diagnostics, hir::Image, phases::Phase};

pub fn phase() -> Phase { Phase::new().and_then(run_system()) }

#[legion::system]
fn run(
    cmd: &mut CommandBuffer,
    #[resource] build_context: &BuildContext,
    #[resource] diags: &mut Diagnostics,
) {
    let src = &build_context.runefile;

    match Document::parse(src) {
        Ok(d) => {
            cmd.exec_mut(move |_, res| {
                let v1 = d.clone().to_v1();
                res.insert(Image(v1.image.clone()));
                res.insert(v1);
            });
        },
        Err(e) => {
            diags.push(parse_failed_diagnostic(e));
        },
    }
}

fn parse_failed_diagnostic(e: serde_yaml::Error) -> Diagnostic<()> {
    let mut diag = Diagnostic::error().with_message(e.to_string());
    if let Some(location) = e.location() {
        let ix = location.index();
        diag = diag.with_labels(vec![Label::primary((), ix..ix)]);
    }
    diag
}
