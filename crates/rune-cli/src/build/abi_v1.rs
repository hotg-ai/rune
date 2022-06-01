use std::path::PathBuf;

use anyhow::{Context, Error};
use query_based_compiler::{
    codegen::{Codegen, CodegenStorage},
    asset_loader::{AssetLoader, DefaultAssetLoader, ReadError},
    im::Vector,
    parse::{Frontend, FrontendStorage},
    BuildConfig, Environment, EnvironmentStorage, FeatureFlags,
};
use salsa::Storage;
use uriparse::URI;

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
        fs: DefaultAssetLoader::default(),
    };

    db.set_config(BuildConfig {
        current_directory: db.current_dir.clone(),
        features: FeatureFlags::stable(),
    });
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
    fs: DefaultAssetLoader,
}

impl salsa::Database for Database {}

impl AssetLoader for Database {
    fn read(&self, uri: &URI<'_>) -> Result<Vector<u8>, ReadError> {
        self.fs.read(uri)
    }
}
