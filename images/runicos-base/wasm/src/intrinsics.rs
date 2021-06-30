//! Functions provided by the host runtime.
//!
//! You probably shouldn't be touching things in here unless you know what you
//! are doing.

use crate::tensor::TensorRepr;

extern "C" {
    /// Invoke the default model with the specified data.
    ///
    /// The model's output will be written to the `output` buffer.
    ///
    /// Model failures will trigger a trap and abort at runtime.
    pub fn tfm_model_invoke(
        model_id: u32,
        input: *const u8,
        input_len: u32,
        output: *mut u8,
        output_len: u32,
    ) -> u32;

    /// Run inference
    ///
    /// The model's output will be written to the `output` buffer.
    ///
    /// Model failures will trigger a trap and abort at runtime.
    pub fn rune_model_infer(
        model_id: u32,
        inputs: *const TensorRepr,
        num_inputs: u32,
        outputs: *mut TensorRepr,
        num_outputs: u32,
    ) -> u32;

    /// Load a model (as a byte buffer) into the runtime, telling it how many
    /// inputs and outputs there will be.
    ///
    /// The return value is a unique identifier that can be used to refer to
    /// the loaded model.
    ///
    /// Model failures will trigger a trap and abort at runtime.
    pub fn tfm_preload_model(
        model: *const u8,
        model_len: u32,
        inputs: u32,
        outputs: u32,
    ) -> u32;

    /// Write some text to the debug console.
    pub fn _debug(msg: *const u8, msg_len: u32) -> u32;

    /// Request a capability with a particular type, yielding a unique handle
    /// that can be used to refer to the capability later on.
    ///
    /// See [`crate::capabilities`] to find out which capabilities are
    /// available.
    pub fn request_capability(capability_type: u32) -> u32;

    /// Set a capability parameter by name.
    ///
    /// Invalid parameters will trigger a trap and abort at runtime.
    pub fn request_capability_set_param(
        capability_id: u32,
        key_ptr: *const u8,
        key_len: u32,
        value_ptr: *const u8,
        value_len: u32,
        value_type: u32,
    ) -> u32;

    /// Ask the runtime to allocate a new output of the specified type.
    ///
    /// See [`crate::outputs`] to find out which outputs are available. Invalid
    /// or unsupported parameters will trigger a trap and abort at runtime.
    pub fn request_output(out_type: u32) -> u32;

    /// Write the result of a pipeline to an output device.
    ///
    /// The contents of the buffer are output-specific. Any errors will trigger
    /// a a trap and abort at runtime.
    pub fn consume_output(output_id: u32, buffer: *const u8, len: u32) -> u32;

    /// Ask a particular capability to fill the `buffer` with input.
    ///
    /// Invalid parameters will trigger a trap and abort at runtime.
    pub fn request_provider_response(
        buffer: *mut u8,
        buffer_len: u32,
        capability_id: u32,
    ) -> u32;
}
