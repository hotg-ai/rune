use std::sync::Arc;

use im::Vector;
use uriparse::URI;

use crate::{type_check::TensorShape, Text};

/// Something which knows how to load proc-blocks.
pub trait ProcBlockRegistry {
    fn load_proc_block_binary(
        &self,
        path: URI<'_>,
    ) -> Result<Vector<u8>, LoadError>;

    /// Load a proc-block and call its `graph()` function to find out what its
    /// inputs and outputs are.
    fn load_graph(&self, path: URI<'_>) -> Result<Tensors, LoadError> {
        let _binary = self.load_proc_block_binary(path)?;
        Err(LoadError::NotSupported)
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct Tensors {
    pub inputs: Vector<(Text, TensorShape)>,
    pub outputs: Vector<(Text, TensorShape)>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum LoadError {
    #[error("Unknown scheme, \"{}\"", scheme)]
    UnknownScheme { scheme: Text },
    #[error("This operation isn't supported")]
    NotSupported,
    #[error(transparent)]
    Other(Arc<dyn std::error::Error + Send + Sync + 'static>),
}

impl LoadError {
    pub fn other(
        error: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        LoadError::Other(Arc::new(error))
    }
}
