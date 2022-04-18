use std::{error::Error, path::Path, sync::Arc};

use im::Vector;

/// An abstract filesystem.
pub trait FileSystem {
    fn read(&self, path: &Path) -> Result<Vector<u8>, ReadError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ReadError {
    #[error("The file wasn't found")]
    NotFound,
    #[error("Unable to read the file")]
    Other(Arc<dyn Error + Send + Sync>),
}

impl PartialEq for ReadError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ReadError::Other(left), ReadError::Other(right)) => {
                Arc::ptr_eq(left, right)
            },
            (ReadError::NotFound, ReadError::NotFound) => true,
            _ => false,
        }
    }
}

impl Eq for ReadError {}
