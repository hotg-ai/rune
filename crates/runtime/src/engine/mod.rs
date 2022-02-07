#[cfg(feature = "wasm3")]
mod wasm3;

#[cfg(feature = "wasmer")]
mod wasmer;

use std::sync::{Arc, Mutex};
use anyhow::Error;
use crate::HostFunctions;

pub trait WebAssemblyEngine {
    fn load(
        wasm: &[u8],
        host_functions: Arc<Mutex<HostFunctions>>,
    ) -> Result<Self, Error>
    where
        Self: Sized;

    fn init(&mut self) -> Result<(), Error>;

    fn call(&mut self) -> Result<(), Error>;
}
