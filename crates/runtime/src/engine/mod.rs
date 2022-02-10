mod host_functions;

#[cfg(feature = "wasm3")]
mod wasm3;

#[cfg(feature = "wasmer")]
mod wasmer;

#[cfg(feature = "wasm3")]
pub use self::wasm3::Wasm3Engine;

use anyhow::Error;
use std::sync::Arc;
use crate::callbacks::Callbacks;

/// A WebAssembly virtual machine that links Rune with
pub(crate) trait WebAssemblyEngine {
    fn load(wasm: &[u8], callbacks: Arc<dyn Callbacks>) -> Result<Self, Error>
    where
        Self: Sized;

    /// Call the `_manifest()` function to initialize the Rune graph.
    fn init(&mut self) -> Result<(), Error>;

    /// Call the `_call()` function to run the Rune.
    fn predict(&mut self) -> Result<(), Error>;
}
