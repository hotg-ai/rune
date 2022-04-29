use std::sync::Arc;

use uriparse::URI;

use crate::{im::Vector, Text};

/// An abstract filesystem.
pub trait FileSystem {
    /// Read a file's contents from somewhere, with the interpretation changing
    /// depending on the URI's scheme.
    fn read(&self, path: &URI<'_>) -> Result<Vector<u8>, ReadError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ReadError {
    #[error("The \"{}\" scheme isn't supported", scheme)]
    UnsupportedScheme { scheme: Text },
    #[error(transparent)]
    Other(Arc<dyn std::error::Error + Send + Sync + 'static>),
}

impl ReadError {
    pub fn other(
        error: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        ReadError::Other(Arc::new(error))
    }
}
