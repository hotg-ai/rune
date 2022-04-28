use std::{
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    sync::Arc,
};

use im::Vector;

/// An abstract filesystem.
pub trait FileSystem {
    fn read(&self, path: &Path) -> Result<Vector<u8>, FileSystemError>;
}

/// An error that may be returned by [`FileSystem`] operations.
#[derive(Debug, Clone, thiserror::Error)]
#[error("Unable to {} \"{}\"", operation, path.display())]
pub struct FileSystemError {
    #[source]
    pub error: Arc<std::io::Error>,
    pub operation: FileSystemOperation,
    pub path: PathBuf,
}

impl FileSystemError {
    pub fn new(
        path: impl Into<PathBuf>,
        operation: FileSystemOperation,
        error: std::io::Error,
    ) -> Self {
        Self {
            error: error.into(),
            operation,
            path: path.into(),
        }
    }

    pub fn read(path: PathBuf, error: std::io::Error) -> Self {
        FileSystemError::new(path, FileSystemOperation::Read, error)
    }
}

impl PartialEq for FileSystemError {
    fn eq(&self, other: &Self) -> bool {
        let FileSystemError {
            error: ref inner,
            operation,
            ref path,
        } = *self;

        if *path != other.path || operation != other.operation {
            return false;
        }

        let kind = inner.kind();

        if kind == std::io::ErrorKind::Other {
            // it's some other type of error. Assume they aren't equal unless
            // they are the same object.
            Arc::ptr_eq(inner, &other.error)
        } else {
            kind == other.error.kind()
        }
    }
}

impl Eq for FileSystemError {}

/// Operations that may be executed as part of implementing [`FileSystem`]
/// methods.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum FileSystemOperation {
    Read,
}

impl Display for FileSystemOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemOperation::Read => "read".fmt(f),
        }
    }
}
