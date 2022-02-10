use crate::{
    engine::{WebAssemblyEngine},
    callbacks::{RuneGraph, Callbacks},
    NodeMetadata, Tensor,
};
use anyhow::Error;
use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
    ops::DerefMut,
};
use arc_swap::ArcSwap;

/// A loaded Rune.
pub struct Runtime {
    state: Arc<State>,
    engine: Box<dyn WebAssemblyEngine>,
}

impl Runtime {
    /// Load a Rune using WASM3 for executing WebAssembly.
    #[cfg(feature = "wasm3")]
    pub fn wasm3(rune: &[u8]) -> Result<Self, Error> {
        let state = Arc::new(State {
            input_tensors: Mutex::default(),
            output_tensors: Mutex::default(),
            capabilities: ArcSwap::default(),
            outputs: ArcSwap::default(),
        });
        let callbacks = Arc::clone(&state) as Arc<dyn Callbacks>;
        let mut engine = crate::engine::Wasm3Engine::load(rune, callbacks)?;

        engine.init()?;

        Ok(Runtime {
            state,
            engine: Box::new(engine),
        })
    }
}

impl Runtime {
    /// Run the Rune.
    pub fn predict(&mut self) -> Result<(), Error> { self.engine.predict() }

    /// Get all input tensors, keyed by capability ID.
    pub fn get_inputs(
        &mut self,
    ) -> impl DerefMut<Target = HashMap<u32, Tensor>> + '_ {
        self.state.input_tensors.lock().unwrap()
    }

    /// Get all output tensors, keyed by output ID.
    pub fn get_outputs(
        &mut self,
    ) -> impl DerefMut<Target = HashMap<u32, Vec<OutputTensor>>> + '_ {
        self.state.output_tensors.lock().unwrap()
    }

    /// Get a mapping from each capability's ID to its metadata.
    pub fn capabilities(&self) -> Arc<HashMap<u32, NodeMetadata>> {
        self.state.capabilities.load_full()
    }

    /// Get a mapping from each output's ID to its metadata.
    pub fn outputs(&self) -> Arc<HashMap<u32, NodeMetadata>> {
        self.state.outputs.load_full()
    }
}

/// State that is shared between the Runtime and the Rune.
struct State {
    input_tensors: Mutex<HashMap<u32, Tensor>>,
    output_tensors: Mutex<HashMap<u32, Vec<OutputTensor>>>,
    // Note: we can't hand out references to our capabilities because the Rune
    // can (theoretically) be re-initialized. The next best thing is to hand
    // out shared pointers that get swapped whenever the `loaded()` method
    // is called.
    capabilities: ArcSwap<HashMap<u32, NodeMetadata>>,
    outputs: ArcSwap<HashMap<u32, NodeMetadata>>,
}

impl Callbacks for State {
    fn loaded(&self, rune: &RuneGraph<'_>) -> Result<(), Error> {
        self.capabilities.store(Arc::new(rune.capabilities.clone()));
        self.outputs.store(Arc::new(rune.outputs.clone()));

        Ok(())
    }

    fn read_capability(
        &self,
        _id: u32,
        _meta: &crate::NodeMetadata,
        _buffer: &mut [u8],
    ) -> Result<(), Error> {
        todo!()
    }

    fn write_output(
        &self,
        _id: u32,
        _meta: &crate::NodeMetadata,
        _data: &[u8],
    ) -> Result<(), Error> {
        todo!()
    }

    fn load_model(
        &self,
        _id: u32,
        _meta: &crate::callbacks::ModelMetadata<'_>,
        _model: &[u8],
    ) -> Result<Box<dyn crate::callbacks::Model>, Error> {
        todo!()
    }

    fn model_infer(
        &self,
        _id: u32,
        _inputs: &[&[u8]],
        _outputs: &mut [&mut [u8]],
    ) -> Result<(), Error> {
        todo!()
    }

    fn get_resource(&self, _name: &str) -> Option<&[u8]> { todo!() }

    fn log(&self, _record: &log::Record<'_>) { todo!() }
}

#[derive(Debug)]
pub enum OutputTensor {
    Tensor(Tensor),
}
