mod host_functions;
#[cfg(feature = "wasm3")]
mod wasm3;
#[cfg(feature = "wasmer")]
mod wasmer;

use std::sync::Arc;

use anyhow::Error;

#[cfg(feature = "wasm3")]
pub(crate) use self::wasm3::Wasm3Engine;
#[cfg(feature = "wasmer")]
pub(crate) use self::wasmer::WasmerEngine;

/// A WebAssembly virtual machine that links Rune with
pub(crate) trait WebAssemblyEngine {
    fn load(
        wasm: &[u8],
        callbacks: Arc<dyn crate::callbacks::Callbacks>,
    ) -> Result<Self, LoadError>
    where
        Self: Sized;

    /// Call the `_manifest()` function to initialize the Rune graph.
    fn init(&mut self) -> Result<(), Error>;

    /// Call the `_call()` function to run the Rune.
    fn predict(&mut self) -> Result<(), Error>;
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LoadError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    #[cfg(feature = "wasmer")]
    WasmerInstantiation(#[from] ::wasmer::InstantiationError),
    #[error(transparent)]
    #[cfg(feature = "wasmer")]
    WasmerCompile(#[from] ::wasmer::CompileError),
}
