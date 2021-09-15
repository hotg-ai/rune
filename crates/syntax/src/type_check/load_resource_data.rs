use std::path::Path;

use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::{Entity, Query, systems::CommandBuffer, world::SubWorld};
use crate::{
    BuildContext, Diagnostics,
    hir::{Name, Resource, ResourceData, ResourceSource},
};

#[legion::system]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    query: &mut Query<(Entity, &Resource, &Name, &Span)>,
    #[resource] diags: &mut Diagnostics,
    #[resource] build_ctx: &mut BuildContext,
) {
    let current_dir = &build_ctx.current_directory;

    query.for_each(world, |(&e, r, n, &s)| match &r.default_value {
        Some(ResourceSource::FromDisk(path)) => {
            match load(current_dir, path, n, s) {
                Ok(data) => cmd.add_component(e, data),
                Err(diag) => diags.push(diag),
            }
        },
        Some(ResourceSource::Inline(data)) => {
            cmd.add_component(e, ResourceData::from(data.as_bytes().to_vec()));
        },
        None => {},
    });
}

fn load(
    current_dir: &Path,
    filename: &Path,
    name: &Name,
    span: Span,
) -> Result<ResourceData, Diagnostic<()>> {
    let full_path = current_dir.join(filename);
    let data = std::fs::read(&full_path).map_err(|e| {
        Diagnostic::error()
            .with_message(format!(
                "Unable to read \"{}\" for \"{}\": {}",
                full_path.display(),
                name,
                e
            ))
            .with_labels(vec![Label::primary((), span)])
    })?;

    Ok(data.into())
}
