use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use im::Vector;

/// An abstract filesystem.
pub trait FileSystem {
    fn read(&self, path: &Path) -> Result<Vector<u8>, ReadError>;
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("Unable to read \"{}\"", path.display())]
pub struct ReadError {
    #[source]
    inner: Arc<std::io::Error>,
    path: PathBuf,
}

impl ReadError {
    pub fn new(path: PathBuf, error: std::io::Error) -> Self {
        Self {
            inner: error.into(),
            path,
        }
    }
}

impl PartialEq for ReadError {
    fn eq(&self, other: &Self) -> bool {
        let ReadError { inner, path } = self;

        if *path != other.path {
            return false;
        }

        let kind = inner.kind();

        if kind == std::io::ErrorKind::Other {
            // it's some other type of error. Assume they aren't equal unless
            // they are the same object.
            Arc::ptr_eq(inner, &other.inner)
        } else {
            kind == other.inner.kind()
        }
    }
}

impl Eq for ReadError {}
