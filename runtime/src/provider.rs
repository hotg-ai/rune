use crate::capability::{Capability, CapabilityParam, CapabilityRequest};
use anyhow::{Context, Error};
use log;
use runic_transform::{Transform, Transformable};
use runic_types::*;

use tflite::{
    ops::builtin::BuiltinOpResolver, FlatBufferModel, Interpreter,
    InterpreterBuilder,
};

pub struct Provider {
    requests: Vec<CapabilityRequest>,
    models: Vec<Vec<u8>>,
}

impl Provider {
    pub fn init() -> Provider {
        return Provider {
            requests: Vec::new(),
            models: Vec::new(),
        };
    }

    pub fn predict_model<T>(
        &mut self,
        idx: u32,
        input: Vec<u8>,
        _value_t: PARAM_TYPE,
    ) -> Result<Vec<T>, Error> {
        log::info!("HAS {} MODELS", self.models.len());
        let model = self
            .models
            .get(idx as usize)
            .with_context(|| format!("Model {} not found", idx))?;

        log::info!("Found model {}", model.len());

        let fb = FlatBufferModel::build_from_buffer(model.to_vec())
            .context("Invalid model provided")?;

        log::info!("Successfully Loaded model as FlatbufferModel");

        let resolver = BuiltinOpResolver::default();

        let builder = InterpreterBuilder::new(fb, resolver)
            .context("Unable to create a model interpreter builder")?;
        let mut interpreter: Interpreter<BuiltinOpResolver> =
            builder
                .build()
                .context("Unable to initialize the model interpreter")?;

        let input =
            Transform::<f32, f32>::from_buffer(&input).map_err(Error::msg)?;

        log::info!("{:?}", interpreter.inputs());
        log::info!("INPUT<{:?}>", input);
        interpreter
            .allocate_tensors()
            .context("Unable to allocate tensors")?;

        let inputs = interpreter.inputs().to_vec();

        let outputs = interpreter.outputs().to_vec();
        let input_index = inputs[0];

        let input_tensor = interpreter
            .tensor_info(input_index)
            .context("Unable to get the input tensor")?;
        log::info!("DIMS = {:?}", input_tensor.dims);

        let output_index = outputs[0];
        let output_tensor = interpreter
            .tensor_info(output_index)
            .context("Unable to get the output tensor")?;
        log::info!("Model loaded with input tensor: {:?}", input_tensor);
        log::info!("Model loaded with output tensor: {:?}", output_tensor);

        let input_tensors: &mut [f32] = interpreter
            .tensor_data_mut(input_index)
            .context("Unable to get the input tensor data")?;

        input_tensors[0] = input[0];

        interpreter.invoke().context("Model execution failed")?;

        let output: &[f32] = interpreter
            .tensor_data(output_index)
            .context("Unable to read the output")?;

        log::info!("Output: {:?}", output);

        Ok(Vec::new())
    }

    pub fn add_model(
        &mut self,
        model_weights: Vec<u8>,
        inputs: u32,
        outputs: u32,
    ) -> u32 {
        let idx = self.models.len();

        self.models.push(model_weights);
        log::info!(
            "Setting Model<{},{}>({})[{}]",
            inputs,
            outputs,
            idx,
            self.models[0].len()
        );
        idx as u32
    }

    pub fn request_capability(&mut self, requested: u32) -> u32 {
        let idx = self.requests.len() as u32;
        let cr = CapabilityRequest::init(runic_types::CAPABILITY::from_u32(
            requested,
        ));

        self.requests.push(cr);
        log::info!(
            "Setting capability({}) {:?}",
            idx,
            runic_types::CAPABILITY::from_u32(requested)
        );
        return idx;
    }

    pub fn set_capability_request_param(
        &mut self,
        request_idx: u32,
        key: String,
        value: Vec<u8>,
        value_t: runic_types::PARAM_TYPE,
    ) {
        match self.requests.get_mut(request_idx as usize) {
            Some(cr) => {
                cr.set_param(key, value, value_t);
                log::info!("Setting params for capability({})", request_idx);
            },
            _ => {
                log::warn!(
                    "Rune called to set param on capability_request({}) that does not exist",
                    request_idx
                );
                return;
            },
        };
    }
}
