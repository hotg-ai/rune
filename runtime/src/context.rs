use crate::{
    capability::{CapabilityParam, CapabilityRequest},
    Environment,
};
use anyhow::{Context as _, Error};
use log::Level;
use runic_types::{OUTPUT, PARAM_TYPE};
use std::collections::HashMap;
use tflite::{
    ops::builtin::BuiltinOpResolver, FlatBufferModel, Interpreter,
    InterpreterBuilder,
};

/// Contextual state associated with a single instance of the
/// [`crate::Runtime`].
pub(crate) struct Context<E> {
    env: E,
    models: HashMap<u32, Interpreter<'static, BuiltinOpResolver>>,
    capabilities: HashMap<u32, CapabilityRequest>,
    outputs: HashMap<u32, OUTPUT>,
    last_id: u32,
}

impl<E: Environment> Context<E> {
    pub fn new(env: E) -> Self {
        Context {
            env,
            last_id: 0,
            models: HashMap::new(),
            capabilities: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    fn next_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }

    pub fn log(&mut self, msg: &str) { self.env.log(msg); }

    /// Load a TensorFlow model and return a unique ID that can be used to refer
    /// to it later.
    pub fn register_model(&mut self, raw: Vec<u8>) -> Result<u32, Error> {
        let model = FlatBufferModel::build_from_buffer(raw)
            .context("Unable to build the model")?;

        let resolver = BuiltinOpResolver::default();

        let builder = InterpreterBuilder::new(model, resolver)
            .context("Unable to create a model interpreter builder")?;
        let mut interpreter = builder
            .build()
            .context("Unable to initialize the model interpreter")?;
        interpreter
            .allocate_tensors()
            .context("Unable to allocate tensors")?;

        let id = self.next_id();

        if log::log_enabled!(Level::Debug) {
            let inputs: Vec<_> = interpreter
                .inputs()
                .iter()
                .filter_map(|ix| interpreter.tensor_info(*ix))
                .collect();
            let outputs: Vec<_> = interpreter
                .inputs()
                .iter()
                .filter_map(|ix| interpreter.tensor_info(*ix))
                .collect();
            log::debug!(
                "Loaded model {} with inputs {:?} and outputs {:?}",
                id,
                inputs,
                outputs
            );
        }

        self.models.insert(id, interpreter);

        Ok(id)
    }

    pub fn request_capability(
        &mut self,
        capability: runic_types::CAPABILITY,
    ) -> u32 {
        let request = CapabilityRequest::new(capability);

        let id = self.next_id();
        self.capabilities.insert(id, request);

        log::debug!("Requested capability {:?} with ID {}", capability, id);

        id
    }

    pub fn set_capability_request_parameter(
        &mut self,
        id: u32,
        key: &str,
        value: Vec<u8>,
        ty: PARAM_TYPE,
    ) -> Result<(), Error> {
        let request = self
            .capabilities
            .get_mut(&id)
            .context("Invalid capability")?;

        let value = CapabilityParam::from_raw(value, ty)
            .context("Invalid capability parameter")?;

        log::debug!("Setting {}={:?} on capability {}", key, value, id);
        request.params.insert(key.to_string(), value);

        Ok(())
    }

    pub fn register_output(&mut self, output: OUTPUT) -> u32 {
        let id = self.next_id();
        log::debug!("Registered the {:?} output as {}", output, id);
        self.outputs.insert(id, output);

        id
    }

    pub fn invoke_capability(
        &mut self,
        id: u32,
        dest: &mut [u8],
    ) -> Result<(), Error> {
        log::debug!("Getting capability {}", id);
        let cap = self.capabilities.get(&id).context("Invalid capability")?;
        log::debug!(
            "Invoking capability {} ({:?}) on a {}-byte buffer",
            id,
            cap.c_type,
            dest.len()
        );

        match cap.c_type {
            runic_types::CAPABILITY::RAND => {
                let rng = self.env.rng().context(
                    "The environment doesn't provide a random number generator",
                )?;

                rng.fill_bytes(dest);

                Ok(())
            },
            other => Err(anyhow::anyhow!(
                "The {:?} capability isn't implemented",
                other
            )),
        }
    }
}
