use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use crate::types::{CapabilityConstructor, OutputConstructor};

/// A table of host functions that will be invoked by the Rune at runtime.
#[wasm_bindgen]
#[derive(Default)]
pub struct Imports {
    capabilities: HashMap<u32, CapabilityConstructor>,
    outputs: HashMap<u32, OutputConstructor>,
}

#[wasm_bindgen]
impl Imports {
    pub fn new() -> Imports { Imports::default() }

    pub fn register_capability(
        &mut self,
        number: u32,
        constructor: CapabilityConstructor,
    ) {
        self.capabilities.insert(number, constructor);
    }

    pub fn register_outputs(
        &mut self,
        number: u32,
        constructor: OutputConstructor,
    ) {
        self.outputs.insert(number, constructor);
    }
}
