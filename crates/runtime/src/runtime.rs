//! Glue code for linking a Rune with the outside world.
//!
//! # Safety
//!
//! This module contains some non-trivial (but sound) `unsafe` code. In general,
//! if you see an `unsafe` block you should refer to this comment to understand
//! the invariants that need to be maintained. Please message @Michael-F-Bryan
//! if you have any questions.
//!
//! The primary source of `unsafe` comes from the fact that our [`State`] is
//! shared between the [`Runtime`] and the [`WebAssemblyEngine`] it contains
//! and both may need to mutate fields at different points in time.
//!
//! We want to simplify the public API by giving the [`Runtime`] methods like
//! [`Runtime::input_tensors()`] which return `&mut` references to the `HashMap`
//! if input tensors, but the naive implementation would be incompatible
//! with this because it'd require wrapping our [`State`]'s fields in
//! `Arc<Mutex<_>>` and returning a [`std::sync::MutexGuard`].
//!
//! However, because we are the authors of the [`Runtime`] and all
//! [`WebAssemblyEngine`] implementations, we have complete control over how our
//! [`State`] is accessed!
//!
//! By making the public API (essentially just our [`Runtime`] type) use
//! `&mut self` methods properly, we can leverage the borrow checker to manage
//! synchronise the access to our [`State`]. No [`std::sync::Mutex`] required.
//!
//! More concretely, this assumes
//!
//! - The [`Runtime`] won't try to access its [`State`] while a
//!   [`WebAssemblyEngine`] method is running
//! - All [`Runtime`] methods and [`WebAssemblyEngine`] implementations are
//!   single-threaded and not re-entrant
//!
//! In the long term I'd *really* like to drop this `unsafe` by changing the API
//! so all memory is owned by the Rune and lives inside WebAssembly linear
//! memory. That way if the caller wants to modify a tensor, they'll need to
//! call a method on the [`Runtime`] which then asks the Rune for a reference to
//! the tensor's buffer.

use std::{cell::UnsafeCell, collections::HashMap, sync::Arc};

use anyhow::{Context, Error};
use log::Record;
use wasmparser::{Parser, Payload};

use crate::{
    callbacks::{Callbacks, Model, ModelMetadata, RuneGraph},
    engine::{LoadError, WebAssemblyEngine},
    outputs::{parse_outputs, OutputTensor},
    NodeMetadata, Tensor,
};

/// A loaded Rune.
pub struct Runtime {
    state: Arc<State>,
    engine: Box<dyn WebAssemblyEngine>,
}

impl Runtime {
    /// Load a Rune, using WASM3 for executing WebAssembly.
    #[cfg(feature = "wasm3")]
    pub fn wasm3(rune: &[u8]) -> Result<Self, LoadError> {
        let state = State::from_wasm_binary(rune);
        let state = Arc::new(state);
        let callbacks = Arc::clone(&state) as Arc<dyn Callbacks>;
        let mut engine = crate::engine::Wasm3Engine::load(rune, callbacks)?;

        engine.init()?;

        Ok(Runtime {
            state,
            engine: Box::new(engine),
        })
    }

    /// Load a Rune, using Wasmer for executing WebAssembly.
    #[cfg(feature = "wasmer")]
    pub fn wasmer(rune: &[u8]) -> Result<Self, LoadError> {
        let state = State::from_wasm_binary(rune);
        let state = Arc::new(state);
        let callbacks = Arc::clone(&state) as Arc<dyn Callbacks>;
        let mut engine = crate::engine::WasmerEngine::load(rune, callbacks)?;

        engine.init()?;

        Ok(Runtime {
            state,
            engine: Box::new(engine),
        })
    }

    #[cfg(feature = "wasmer")]
    pub fn wasmer_from_module(
        store: wasmer::Store,
        module: wasmer::Module,
    ) -> Result<Self, LoadError> {
        let resource_sections = module.custom_sections(".rune_resource");
        let state = State::new(resource_sections);
        let state = Arc::new(state);
        let callbacks = Arc::clone(&state) as Arc<dyn Callbacks>;
        let mut engine =
            crate::engine::WasmerEngine::from_module(store, module, callbacks)?;

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
    pub fn input_tensors(&mut self) -> &mut HashMap<u32, Tensor> {
        unsafe { self.state.input_tensors() }
    }

    /// Get all output tensors, keyed by output ID.
    pub fn output_tensors(&self) -> &HashMap<u32, Vec<OutputTensor>> {
        unsafe { self.state.output_tensors() }
    }

    /// Get a mapping from each capability's ID to its metadata.
    pub fn capabilities(&self) -> &HashMap<u32, NodeMetadata> {
        unsafe { self.state.capabilities() }
    }

    /// Get a mapping from each output's ID to its metadata.
    pub fn outputs(&self) -> &HashMap<u32, NodeMetadata> {
        unsafe { self.state.outputs() }
    }

    pub fn set_model_handler<F>(&mut self, load_model: F)
    where
        F: Fn(u32, &ModelMetadata<'_>, &[u8]) -> Result<Box<dyn Model>, Error>,
        F: Sync + Send + 'static,
    {
        unsafe { self.state.set_model_handler(load_model) }
    }

    pub fn set_logger<L>(&mut self, log: L)
    where
        L: Fn(&Record<'_>),
        L: Send + Sync + 'static,
    {
        unsafe { self.state.set_logger(log) }
    }

    pub fn resources(&mut self) -> &mut HashMap<String, Vec<u8>> {
        unsafe { self.state.resources() }
    }
}

/// State that is shared between the Runtime and the Rune.
struct State {
    input_tensors: UnsafeCell<HashMap<u32, Tensor>>,
    output_tensors: UnsafeCell<HashMap<u32, Vec<OutputTensor>>>,
    capabilities: UnsafeCell<HashMap<u32, NodeMetadata>>,
    outputs: UnsafeCell<HashMap<u32, NodeMetadata>>,
    load_model: UnsafeCell<
        Box<
            dyn Fn(
                    u32,
                    &ModelMetadata<'_>,
                    &[u8],
                ) -> Result<Box<dyn Model>, Error>
                + Sync
                + Send,
        >,
    >,
    log: UnsafeCell<Box<dyn Fn(&Record<'_>) + Send + Sync>>,
    resources: UnsafeCell<HashMap<String, Vec<u8>>>,
}

impl State {
    /// Construct the `State` by extracting resources from a WebAssembly
    /// binary's custom sections.
    fn from_wasm_binary(wasm: &[u8]) -> Self {
        let _s = State::default();

        let resource_sections =
            Parser::default().parse_all(wasm).filter_map(|p| match p {
                Ok(Payload::CustomSection {
                    name: ".rune_resource",
                    data,
                    ..
                }) => Some(data),
                _ => None,
            });

        State::new(resource_sections)
    }

    fn new<'a, A>(resource_sections: impl Iterator<Item = A>) -> Self
    where
        A: AsRef<[u8]>,
    {
        let s = State::default();

        for section in resource_sections {
            let mut section = section.as_ref();

            while let Some((resource_name, value, rest)) =
                hotg_rune_core::decode_inline_resource(section)
            {
                // Safety: fine because we are the only ones with access to
                // State at the moment.
                let resources = unsafe { s.resources() };
                resources.insert(resource_name.to_string(), value.to_vec());
                section = rest;
            }
        }

        s
    }

    unsafe fn outputs(&self) -> &HashMap<u32, NodeMetadata> {
        &*self.outputs.get()
    }

    unsafe fn capabilities(&self) -> &HashMap<u32, NodeMetadata> {
        &*self.capabilities.get()
    }

    unsafe fn output_tensors(&self) -> &HashMap<u32, Vec<OutputTensor>> {
        &*self.output_tensors.get()
    }

    unsafe fn input_tensors(&self) -> &mut HashMap<u32, Tensor> {
        &mut *self.input_tensors.get()
    }

    unsafe fn resources(&self) -> &mut HashMap<String, Vec<u8>> {
        &mut *self.resources.get()
    }

    unsafe fn set_logger<L>(&self, log: L)
    where
        L: Fn(&Record<'_>),
        L: Send + Sync + 'static,
    {
        *self.log.get() = Box::new(log);
    }

    unsafe fn set_model_handler<F>(&self, load_model: F)
    where
        F: Fn(u32, &ModelMetadata<'_>, &[u8]) -> Result<Box<dyn Model>, Error>,
        F: Sync + Send + 'static,
    {
        *self.load_model.get() = Box::new(load_model);
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            input_tensors: UnsafeCell::default(),
            output_tensors: UnsafeCell::default(),
            capabilities: UnsafeCell::default(),
            outputs: UnsafeCell::default(),
            load_model: UnsafeCell::new(Box::new(
                crate::models::default_model_handler,
            )),
            log: UnsafeCell::new(Box::new(|_| {})),
            resources: UnsafeCell::default(),
        }
    }
}

impl Callbacks for State {
    fn loaded(&self, rune: &RuneGraph<'_>) -> Result<(), Error> {
        log::debug!("Loaded {:?}", rune);

        // Safety: see the safety comments on State
        let capabilities = unsafe { &mut *self.capabilities.get() };
        let outputs = unsafe { &mut *self.outputs.get() };

        *capabilities = rune.capabilities.clone();
        *outputs = rune.outputs.clone();

        Ok(())
    }

    fn read_capability(
        &self,
        id: u32,
        meta: &NodeMetadata,
        buffer: &mut [u8],
    ) -> Result<usize, Error> {
        // Safety: see the safety comments on State
        let inputs = unsafe { &*self.input_tensors.get() };
        let tensor = inputs.get(&id).with_context(|| {
            format!(
                "No input tensor provided for the \"{}\" capability with ID {}",
                meta.kind, id
            )
        })?;

        let src = tensor.buffer();

        if src.len() != buffer.len() {
            anyhow::bail!(
                "The Rune provided a {} byte buffer, but the input tensor is \
                 {} ({} bytes)",
                buffer.len(),
                tensor.shape(),
                src.len(),
            );
        }

        buffer.copy_from_slice(src);

        Ok(src.len())
    }

    fn write_output(
        &self,
        id: u32,
        meta: &NodeMetadata,
        data: &[u8],
    ) -> Result<(), Error> {
        // Safety: see the safety comments on State
        let outputs = unsafe { &mut *self.output_tensors.get() };

        let parsed = parse_outputs(meta, data).with_context(|| {
            format!(
                "Unable to parse the \"{}\" output with ID {}",
                meta.kind, id
            )
        })?;

        outputs.insert(id, parsed);

        Ok(())
    }

    fn load_model(
        &self,
        id: u32,
        meta: &ModelMetadata<'_>,
        model: &[u8],
    ) -> Result<Box<dyn crate::callbacks::Model>, Error> {
        // Safety: see the safety comments on State
        let load_model = unsafe { &*self.load_model.get() };
        load_model(id, meta, model)
    }

    fn get_resource(&self, name: &str) -> Option<&[u8]> {
        // Safety: see the safety comments on State
        let resources = unsafe { &*self.resources.get() };

        resources.get(name).map(|s| s.as_slice())
    }

    fn log(&self, record: &Record<'_>) {
        // Safety: see the safety comments on State
        let log = unsafe { &*self.log.get() };
        log(record);
    }
}

// Safety: see comments on the `State` type itself.
unsafe impl Sync for State {}
