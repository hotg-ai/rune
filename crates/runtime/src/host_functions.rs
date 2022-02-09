use std::{
    sync::Arc,
    collections::HashMap,
    io::{Read, Cursor},
};

use anyhow::{Error, Context};
use hotg_rune_core::{Shape, SerializableRecord};

use crate::{
    callbacks::{Callbacks, NodeMetadata, ModelMetadata, Model},
    RuneGraph,
};

pub struct HostFunctions {
    next: u32,
    callbacks: Arc<dyn Callbacks>,
    capabilities: HashMap<u32, NodeMetadata>,
    outputs: HashMap<u32, NodeMetadata>,
    resources: HashMap<u32, Box<dyn Read>>,
    models: HashMap<u32, Box<dyn Model>>,
}

impl HostFunctions {
    pub fn new(callbacks: Arc<dyn Callbacks>) -> Self {
        HostFunctions {
            callbacks,
            next: 1,
            capabilities: HashMap::new(),
            outputs: HashMap::new(),
            resources: HashMap::new(),
            models: HashMap::new(),
        }
    }

    pub fn graph(&self) -> RuneGraph<'_> {
        RuneGraph {
            capabilities: &self.capabilities,
            outputs: &self.outputs,
        }
    }

    pub(crate) fn model_by_id(&mut self, id: u32) -> Option<&mut dyn Model> {
        self.models.get_mut(&id).map(|m| &mut **m)
    }

    fn next_id(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        id
    }

    pub fn debug(&self, message: &str) -> Result<(), Error> {
        log::debug!("Received message: {}", message);

        match serde_json::from_str::<SerializableRecord>(message) {
            Ok(record) => {
                record.with_record(|r| self.callbacks.log(r));
            },
            Err(e) => {
                log::warn!(
                    "Unable to deserialize {:?} as a log message: {}",
                    message,
                    e
                );
            },
        }

        Ok(())
    }

    pub fn request_capability(
        &mut self,
        capability_type: u32,
    ) -> Result<u32, Error> {
        let id = self.next_id();

        let capability_name =
            hotg_rune_core::capabilities::name(capability_type).with_context(
                || format!("Unknown capability type: {}", capability_type),
            )?;

        let meta = NodeMetadata {
            kind: capability_name.to_string(),
            arguments: HashMap::new(),
        };
        self.capabilities.insert(id, meta);

        Ok(id)
    }

    pub fn request_capability_set_param(
        &mut self,
        capability_id: u32,
        key: &str,
        value: impl Into<String>,
    ) -> Result<(), Error> {
        let meta =
            self.capabilities.get_mut(&capability_id).with_context(|| {
                format!(
                "Trying to set \"{}\" on non-existent capability with ID {}",
                key, capability_id
            )
            })?;
        meta.arguments.insert(key.to_string(), value.into());

        Ok(())
    }

    pub fn request_provider_response(
        &self,
        capability_id: u32,
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        let meta =
            self.capabilities.get(&capability_id).with_context(|| {
                format!(
                    "Tried to read from non-existent capability with ID {}",
                    capability_id
                )
            })?;

        self.callbacks
            .read_capability(capability_id, meta, buffer)
            .context("Unable to read from the capability")?;

        Ok(())
    }

    pub fn tfm_model_invoke(&self) -> Result<(), Error> {
        anyhow::bail!("This feature has been removed")
    }

    pub fn tfm_preload_model(&self) -> Result<(), Error> {
        anyhow::bail!("This feature has been removed")
    }

    pub fn rune_model_load(
        &mut self,
        mimetype: &str,
        model: &[u8],
        inputs: &[Shape<'_>],
        outputs: &[Shape<'_>],
    ) -> Result<u32, Error> {
        let id = self.next_id();

        let meta = ModelMetadata {
            mimetype,
            inputs,
            outputs,
        };

        let model = self.callbacks.load_model(id, &meta, model)
        .with_context(|| format!("Unable to load the \"{}\" model with inputs {:?} and outputs {:?}", mimetype, inputs, outputs))?;

        self.models.insert(id, model);

        Ok(id)
    }

    pub fn rune_model_infer(
        &mut self,
        model_id: u32,
        inputs: &[&[u8]],
        outputs: &mut [&mut [u8]],
    ) -> Result<(), Error> {
        let model = self.models.get_mut(&model_id).with_context(|| {
            format!("Tried to access non-existent model with ID {}", model_id)
        })?;

        model.infer(inputs, outputs)?;

        Ok(())
    }

    pub fn request_output(&mut self, output_type: u32) -> Result<u32, Error> {
        let id = self.next_id();

        let output_name = hotg_rune_core::outputs::name(output_type)
            .with_context(|| format!("Unknown output type: {}", output_type))?;

        let meta = NodeMetadata {
            kind: output_name.to_string(),
            arguments: HashMap::new(),
        };
        self.capabilities.insert(id, meta);

        Ok(id)
    }

    pub fn consume_output(
        &mut self,
        output_id: u32,
        data: &[u8],
    ) -> Result<(), Error> {
        let metadata = self.outputs.get(&output_id).with_context(|| {
            format!(
                "Tried to write to non-existent output with ID {}",
                output_id
            )
        })?;

        self.callbacks
            .write_output(output_id, metadata, data)
            .context("Writing output failed")?;

        Ok(())
    }

    pub fn rune_resource_open(&mut self, name: &str) -> Result<u32, Error> {
        let resource = self
            .callbacks
            .get_resource(name)
            .with_context(|| format!("No resource named \"{}\"", name))?;

        let reader = Box::new(Cursor::new(resource.to_vec()));
        let id = self.next_id();

        self.resources.insert(id, reader);

        Ok(id)
    }

    pub fn rune_resource_read(
        &mut self,
        resource_id: u32,
        buffer: &mut [u8],
    ) -> Result<u32, Error> {
        let resource =
            self.resources.get_mut(&resource_id).with_context(|| {
                format!(
                    "Tried to read from non-existed resource with ID {}",
                    resource_id
                )
            })?;

        let bytes_read = resource
            .read(buffer)
            .context("Unable to read from the resource")?;

        Ok(bytes_read as u32)
    }

    pub fn rune_resource_close(
        &mut self,
        resource_id: u32,
    ) -> Result<(), Error> {
        let _ = self.resources.remove(&resource_id).with_context(|| {
            format!(
                "Tried to close non-existed resource with ID {}",
                resource_id
            )
        })?;

        Ok(())
    }
}
