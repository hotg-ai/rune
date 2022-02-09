#[cfg(feature = "wasm3")]
mod wasm3;

#[cfg(feature = "wasmer")]
mod wasmer;

use std::sync::{Arc, Mutex};
use anyhow::Error;
use crate::HostFunctions;

pub(crate) trait WebAssemblyEngine {
    fn load(
        wasm: &[u8],
        host_functions: Arc<Mutex<HostFunctions>>,
    ) -> Result<Self, Error>
    where
        Self: Sized;

    /// Call the `_manifest()` function to initialize the Rune graph.
    fn init(&mut self) -> Result<(), Error>;

    /// Call the `_call()` function to run the Rune.
    fn predict(&mut self) -> Result<(), Error>;
}
