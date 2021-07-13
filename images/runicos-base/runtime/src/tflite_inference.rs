use log::Level;
use anyhow::Context;
use tflite::{
    FlatBufferModel, Interpreter, InterpreterBuilder, op_resolver::OpResolver,
    ops::builtin::BuiltinOpResolver,
};
use anyhow::Error;
use crate::Model;

pub(crate) fn initialize_model(raw: &[u8]) -> Result<Box<dyn Model>, Error> {
    let model = FlatBufferModel::build_from_buffer(raw.to_vec())
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

    if log::log_enabled!(Level::Debug) {
        let inputs: Vec<_> = interpreter
            .inputs()
            .iter()
            .filter_map(|ix| interpreter.tensor_info(*ix))
            .collect();
        let outputs: Vec<_> = interpreter
            .outputs()
            .iter()
            .filter_map(|ix| interpreter.tensor_info(*ix))
            .collect();
        log::debug!(
            "Loaded model with inputs {:?} and outputs {:?}",
            inputs,
            outputs
        );
    }

    Ok(Box::new(interpreter))
}

impl<R> Model for Interpreter<'static, R>
where
    R: OpResolver + 'static,
{
    fn infer(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error> {
        let tensor_inputs = self.inputs();
        anyhow::ensure!(
            tensor_inputs.len() == 1,
            "We can't handle models with less/more than 1 input"
        );
        let input_index = tensor_inputs[0];

        let buffer = self
            .tensor_buffer_mut(input_index)
            .context("Unable to get the input buffer")?;

        if input.len() != buffer.len() {
            log::warn!(
                        "The input vector for the model is {} bytes long but the tensor expects {}",
                        input.len(),
                        buffer.len(),
                    );
        }
        let len = std::cmp::min(input.len(), buffer.len());
        buffer[..len].copy_from_slice(&input[..len]);

        log::debug!("Model received {} bytes", buffer.len());
        log::trace!("Model input: {:?}", &buffer[..len]);

        self.invoke().context("Calling the model failed")?;

        let tensor_outputs = self.outputs();
        anyhow::ensure!(
            tensor_outputs.len() == 1,
            "We can't handle models with less/more than 1 output"
        );
        let output_index = tensor_outputs[0];
        let buffer = self
            .tensor_buffer(output_index)
            .context("Unable to get the output buffer")?;

        log::debug!("Model Output: {:?}", buffer);

        anyhow::ensure!(buffer.len() == output.len());
        output.copy_from_slice(buffer);

        Ok(())
    }
}
