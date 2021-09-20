use std::path::Path;
use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::{Entity, systems::CommandBuffer};
use crate::{
    BuildContext, Diagnostics,
    lowering::{Name, Resource, ResourceSource},
    type_check::ResourceData,
};

#[legion::system(for_each)]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    #[resource] diags: &mut Diagnostics,
    #[resource] build_ctx: &BuildContext,
    &entity: &Entity,
    name: &Name,
    resource: &Resource,
    &span: &Span,
) {
    let current_dir = &build_ctx.current_directory;

    match &resource.default_value {
        Some(ResourceSource::FromDisk(path)) => {
            match load(current_dir, path, name, span) {
                Ok(data) => cmd.add_component(entity, ResourceData::from(data)),
                Err(diag) => diags.push(diag),
            }
        },
        Some(ResourceSource::Inline(data)) => {
            cmd.add_component(entity, ResourceData::from(data.as_bytes()));
        },
        None => {},
    }
}

pub(crate) fn load(
    current_dir: &Path,
    filename: &Path,
    name: &Name,
    span: Span,
) -> Result<Vec<u8>, Diagnostic<()>> {
    let full_path = current_dir.join(filename);
    let data = std::fs::read(&full_path)
        .map_err(|e| read_failed_diagnostic(&full_path, name, e, span))?;

    Ok(data)
}

fn read_failed_diagnostic(
    full_path: &Path,
    name: &Name,
    e: std::io::Error,
    span: Span,
) -> Diagnostic<()> {
    let msg = format!(
        "Unable to read \"{}\" for \"{}\": {}",
        full_path.display(),
        name,
        e
    );

    Diagnostic::error()
        .with_message(msg)
        .with_labels(vec![Label::primary((), span)])
}
