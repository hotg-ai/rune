use std::path::{Component, Path, PathBuf};

use reqwest::blocking::Client;
use uriparse::{Scheme, URI};

use crate::{
    filesystem::{FileSystem, ReadError, WapmUri},
    im::Vector,
};

#[derive(Debug, Clone)]
pub struct StandardFileSystem {
    client: Client,
    root_directory: PathBuf,
}

impl StandardFileSystem {
    pub fn new(root_directory: impl Into<PathBuf>) -> Self {
        StandardFileSystem {
            client: Client::default(),
            root_directory: root_directory.into(),
        }
    }
}

impl Default for StandardFileSystem {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_default();
        StandardFileSystem::new(current_dir)
    }
}

impl FileSystem for StandardFileSystem {
    #[tracing::instrument(skip(self), err)]
    fn read(&self, uri: &URI<'_>) -> Result<Vector<u8>, ReadError> {
        match uri.scheme() {
            Scheme::HTTP | Scheme::HTTPS => {
                http_file(&self.client, uri).map_err(ReadError::from)
            },
            Scheme::File => local_file(&self.root_directory, uri.path()),
            Scheme::Unregistered(u) if u.as_str().is_empty() => {
                local_file(&self.root_directory, uri.path())
            },
            Scheme::Unregistered(u) if u.as_str() == "wapm" => {
                let uri: WapmUri = uri.try_into().map_err(ReadError::other)?;
                todo!()
            },
            other => Err(ReadError::UnsupportedScheme {
                scheme: other.as_str().into(),
            }),
        }
    }
}

#[tracing::instrument(skip_all)]
fn http_file(
    client: &Client,
    url: &URI<'_>,
) -> Result<Vector<u8>, reqwest::Error> {
    tracing::info!(%url, "Downloading");
    let response = client.get(url.to_string()).send()?.error_for_status()?;

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
