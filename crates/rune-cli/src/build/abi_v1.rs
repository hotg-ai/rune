use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Error};
use query_based_compiler::{
    codegen::{Codegen, CodegenStorage},
    im::Vector,
    parse::{Frontend, FrontendStorage},
    EnvironmentStorage, FileSystem, ReadError,
};
use salsa::Storage;
use uriparse::{Scheme, URI};

use crate::{Build, Unstable};

pub(crate) fn execute(build: Build, unstable: Unstable) -> Result<(), Error> {
    if !unstable.unstable {
        anyhow::bail!("Building with the new ABI is still experimental. Please use the `--unstable` flag.");
    }

    let runefile =
        std::fs::read_to_string(&build.runefile).with_context(|| {
            format!("Unable to read \"{}\"", build.runefile.display())
        })?;
    let name = build.name()?;

    let mut db = Database {
        storage: Storage::default(),
        current_dir: build.current_directory()?,
    };

    db.set_src(runefile.into());
    let archive = db.rune_archive()?;

    let dest = build
        .output
        .unwrap_or_else(|| db.current_dir.join(&name).with_extension("rune"));

    tracing::info!(path = %dest.display(), "Saving the compiled Rune");

    std::fs::write(&dest, &archive)
        .with_context(|| format!("Unable to save to \"{}\"", dest.display()))?;

    Ok(())
}

#[salsa::database(CodegenStorage, EnvironmentStorage, FrontendStorage)]
struct Database {
    storage: Storage<Self>,
    current_dir: PathBuf,
}

impl salsa::Database for Database {}

impl FileSystem for Database {
    fn read(&self, path: &URI<'_>) -> Result<Vector<u8>, ReadError> {
        match path.scheme() {
            Scheme::FileSystem => read_file(path.path()),
            Scheme::Unregistered(u) if u.as_str().is_empty() => {
                read_file(path.path())
            },

            other => Err(ReadError::UnsupportedScheme {
                scheme: other.as_str().into(),
            }),
        }
    }
}

fn read_file(path: &uriparse::Path<'_>) -> Result<Vector<u8>, ReadError> {
    let mut full_path = PathBuf::new();

    if path.is_absolute() {
        full_path.push(std::path::Component::RootDir);
    }

    for segment in path.segments() {
        full_path.push(segment.as_str());
    }

    std::fs::read(&full_path)
        .map(Vector::from)
        .map_err(|e| ReadError::Other(Arc::new(e) as Arc<_>))
}
