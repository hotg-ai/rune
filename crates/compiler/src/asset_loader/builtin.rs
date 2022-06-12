use std::path::{Component, Path, PathBuf};

use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;
use uriparse::{Scheme, URI};

use crate::{
    asset_loader::{
        builtin::lookup_package::{
            LookupPackageGetPackageVersion,
            LookupPackageGetPackageVersionModules,
        },
        AssetLoader, ReadError, WapmUri,
    },
    im::Vector,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/asset_loader/schema.graphql",
    query_path = "src/asset_loader/queries.graphql",
    response_derives = "Debug,Serialize"
)]
pub struct LookupPackage;

const WAPM_REGISTRY: &str = "https://registry.wapm.io/graphql";

/// An [`AssetLoader`] that loads assets using the basic solution.
#[derive(Debug, Clone)]
pub struct DefaultAssetLoader {
    client: Client,
    root_directory: PathBuf,
}

impl DefaultAssetLoader {
    pub fn new(root_directory: impl Into<PathBuf>) -> Self {
        DefaultAssetLoader {
            client: Client::builder().danger_accept_invalid_certs(true).build().or::<reqwest::Client>(Ok(Client::new())).unwrap(),
            root_directory: root_directory.into(),
        }
    }
}

impl Default for DefaultAssetLoader {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_default();
        DefaultAssetLoader::new(current_dir)
    }
}

impl AssetLoader for DefaultAssetLoader {
    #[tracing::instrument(skip(self), err)]
    fn read(&self, uri: &URI<'_>) -> Result<Vector<u8>, ReadError> {
        match uri.scheme() {
            Scheme::HTTP | Scheme::HTTPS => {
                let url = uri.to_string();
                http_file(&self.client, &url).map_err(ReadError::from)
            },
            Scheme::File => local_file(&self.root_directory, uri.path()),
            Scheme::Unregistered(u) if u.as_str().is_empty() => {
                local_file(&self.root_directory, uri.path())
            },
            Scheme::Unregistered(u) if u.as_str() == "wapm" => {
                let uri = uri.try_into().map_err(ReadError::other)?;
                wapm_file(&self.client, uri).map_err(ReadError::other)
            },
            other => Err(ReadError::UnsupportedScheme {
                scheme: other.as_str().into(),
            }),
        }
    }
}

#[tracing::instrument(skip_all)]
fn wapm_file(
    client: &Client,
    uri: WapmUri,
) -> Result<Vector<u8>, WapmDownloadError> {
    let WapmUri {
        namespace,
        package_name,
        version,
        module,
    } = uri;
    tracing::debug!(%namespace, %package_name, ?version, "Querying WAPM's GraphQL API");

    let variables = lookup_package::Variables {
        name: format!("{namespace}/{package_name}"),
        version,
    };

    let Response { data, errors } =
        graphql_client::reqwest::post_graphql_blocking::<LookupPackage, _>(
            client,
            WAPM_REGISTRY,
            variables,
        )?;

    if let Some(mut errors) = errors {
        if !errors.is_empty() {
            return Err(WapmDownloadError::GraphQL(errors.remove(0)));
        }
    }

    let LookupPackageGetPackageVersion { version, modules } = data
        .and_then(|r| r.get_package_version)
        .ok_or(WapmDownloadError::PackageNotFound)?;

    tracing::debug!(%version, ?modules, "Found a compatible package");

    let LookupPackageGetPackageVersionModules { public_url, .. } = match module
    {
        Some(module) => match modules.iter().find(|v| v.name == module) {
            Some(v) => v,
            None => {
                return Err(WapmDownloadError::ModuleNotFound {
                    requested: module,
                    names: modules.iter().map(|v| v.name.clone()).collect(),
                })
            },
        },
        None => match modules.as_slice() {
            [] => return Err(WapmDownloadError::EmptyPackage),
            [v] => v,
            [..] => {
                return Err(WapmDownloadError::MultipleModules {
                    names: modules.iter().map(|v| v.name.clone()).collect(),
                })
            },
        },
    };

    http_file(client, public_url).map_err(WapmDownloadError::Http)
}

#[derive(Debug, thiserror::Error)]
enum WapmDownloadError {
    #[error("The request failed")]
    Http(#[from] reqwest::Error),
    #[error("GraphQL")]
    GraphQL(graphql_client::Error),
    #[error("The package wasn't found")]
    PackageNotFound,
    #[error("The package contains multiple modules, but the caller didn't say which to use")]
    MultipleModules { names: Vec<String> },
    #[error("The package is empty")]
    EmptyPackage,
    #[error("Couldn't find the \"{requested}\" module in {names:?}")]
    ModuleNotFound {
        requested: String,
        names: Vec<String>,
    },
}

#[tracing::instrument(skip_all)]
fn http_file(client: &Client, url: &str) -> Result<Vector<u8>, reqwest::Error> {
    tracing::info!(%url, "Downloading");

    let response = client.get(url).send()?.error_for_status()?;
    let status_code = response.status();
    let body = response.bytes()?;
    let body = Vector::from(&*body);

    tracing::info!(
        status_code = status_code.as_u16(),
        status = %status_code,
        bytes_read = body.len(),
        "Downloaded",
    );

    Ok(body)
}

#[tracing::instrument(skip_all)]
fn local_file(
    root: &Path,
    uri: &uriparse::Path<'_>,
) -> Result<Vector<u8>, ReadError> {
    let mut path = if uri.is_absolute() {
        PathBuf::from(Component::RootDir.as_os_str())
    } else {
        root.to_path_buf()
    };

    for segment in uri.segments() {
        path.push(segment.as_str());
    }

    tracing::debug!(path = %path.display(), "Reading a file from disk");
    std::fs::read(&path)
        .map(Vector::from)
        .map_err(ReadError::from)
}

impl From<reqwest::Error> for ReadError {
    fn from(e: reqwest::Error) -> Self {
        if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
            ReadError::NotFound
        } else {
            ReadError::other(e)
        }
    }
}
