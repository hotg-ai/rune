use std::sync::Arc;

use im::Vector;
use uriparse::URI;

use crate::Text;

/// An abstract filesystem.
pub trait FileSystem {
    /// Read a file's contents from somewhere, with the interpretation changing
    /// depending on the URI's scheme.
    fn read(&self, path: &URI<'_>) -> Result<Vector<u8>, ReadError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ReadError {
    #[error("Unknown scheme, \"{}\"", scheme)]
    UnknownScheme { scheme: Text },
    #[error("This operation isn't supported")]
    NotSupported,
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
