use std::{convert::TryInto, fmt::Display, path::PathBuf, sync::Arc};

use anyhow::{Context, Error};
use query_based_compiler::{
    codegen::{Codegen, CodegenStorage},
    im::Vector,
    parse::{Frontend, FrontendStorage},
    BuildConfig, Environment, EnvironmentStorage, FeatureFlags, FileSystem,
    ReadError,
};
use salsa::Storage;
use serde::Deserialize;
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
}

impl salsa::Database for Database {}

impl FileSystem for Database {
    fn read(&self, path: &URI<'_>) -> Result<Vector<u8>, ReadError> {
        match path.scheme() {
            Scheme::FileSystem | Scheme::File => read_file(path.path()),
            Scheme::HTTP | Scheme::HTTPS => download_from_the_internet(path),
            Scheme::Unregistered(u) if u.as_str().is_empty() => {
                read_file(path.path())
            },
            Scheme::Unregistered(u) if u.as_str() == "wapm" => {
                download_from_wapm(path)
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

#[tracing::instrument]
fn download_from_wapm(uri: &URI<'_>) -> Result<Vector<u8>, ReadError> {
    let (namespace, package_name) = match uri.path().segments() {
        [ns, pkg] => (ns.as_str(), pkg.as_str()),
        _ => {
            return Err(ReadError::other(MalformedPackagePath {
                path: uri.path().clone().into_owned(),
            }))
        },
    };

    // https://registry-cdn.wapm.io/contents/hotg-ai/softmax/0.12.0/softmax.wasm
    let query = uri.query().map(|q| q.as_str()).unwrap_or_default();
    let query = queryst::parse(query).map_err(|e| ReadError::msg(e.message))?;
    let query_params: QueryParams =
        QueryParams::deserialize(&query).map_err(ReadError::other)?;
    let version = query_params
        .version
        .ok_or_else(|| ReadError::msg("No version specified"))?;

    let wapm_url = format!("https://registry-cdn.wapm.io/contents/{namespace}/{package_name}/{version}/{package_name}.wasm");
    let wapm_url = wapm_url.as_str().try_into().map_err(ReadError::other)?;

    download_from_the_internet(&wapm_url)
}

#[tracing::instrument]
fn download_from_the_internet(uri: &URI<'_>) -> Result<Vector<u8>, ReadError> {
    let url = uri.to_string();
    let body = reqwest::blocking::get(&url)
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.bytes())
        .map_err(ReadError::other)?;

    Ok(body.as_ref().into())
}

#[derive(Debug, serde::Deserialize)]
struct QueryParams {
    version: Option<String>,
}

#[derive(Debug)]
struct MalformedPackagePath {
    path: uriparse::Path<'static>,
}

impl Display for MalformedPackagePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to determine the package name and namespace from \"{}\". Expected something like <namespace>/<name>", self.path)
    }
}

impl std::error::Error for MalformedPackagePath {}
