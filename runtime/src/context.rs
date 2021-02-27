use std::collections::HashMap;

use anyhow::{Context as _, Error};
use tflite::{
    ops::builtin::BuiltinOpResolver, FlatBufferModel, Interpreter,
    InterpreterBuilder,
};

use crate::Environment;

/// Contextual state associated with a single instance of the [`Runtime`].
pub(crate) struct Context<E> {
    env: E,
    models: HashMap<u32, Interpreter<'static, BuiltinOpResolver>>,
    last_id: u32,
}

impl<E: Environment> Context<E> {
    pub fn new(env: E) -> Self {
        Context {
            env,
            models: HashMap::new(),
            last_id: 0,
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
        self.models.insert(id, interpreter);

        Ok(id)
    }
}
