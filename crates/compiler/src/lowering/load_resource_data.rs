use std::{
    fs::{DirEntry, File},
    io::{Cursor, ErrorKind, Seek, Write},
    path::Path,
};

use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::{systems::CommandBuffer, Entity};
use zip::{write::FileOptions, ZipWriter};

use crate::{
    lowering::{Name, Resource, ResourceData, ResourceSource},
    BuildContext, Diagnostics,
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
            cmd.add_component(entity, ResourceData::from(data.clone()));
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

    let loaded = if full_path.is_dir() {
        load_directory(&full_path)
    } else {
        std::fs::read(&full_path)
    };

    loaded.map_err(|e| read_failed_diagnostic(&full_path, name, e, span))
}

fn load_directory(full_path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = Cursor::new(Vec::new());

    let mut archive = ZipWriter::new(&mut buffer);

    for entry in full_path.read_dir()? {
        let entry = entry?;
        append_entry(&mut archive, full_path, entry)?;
    }

    archive.finish()?;
    drop(archive);

    Ok(buffer.into_inner())
}

fn append_entry(
    archive: &mut ZipWriter<impl Write + Seek>,
    root: &Path,
    entry: DirEntry,
) -> Result<(), std::io::Error> {
    let path = entry.path();
    let relative_path = path
        .strip_prefix(root)
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
    let relative_path = relative_path.display().to_string();

    let meta = entry.metadata()?;
    let options = FileOptions::default();

    if meta.is_dir() {
        archive.add_directory(relative_path, options)?;

        for entry in path.read_dir()? {
            let entry = entry?;
            append_entry(archive, root, entry)?;
        }
    } else {
        archive.start_file(relative_path, options)?;
        let mut f = File::open(path)?;
        std::io::copy(&mut f, archive)?;
    }

    Ok(())
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
