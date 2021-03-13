//! Functions provided by the host runtime.
//!
//! You probably shouldn't be touching things in here unless you know what you
//! are doing.

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
    /// See [`crate::CAPABILITY`] to find out which capabilities are available.
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

    /// Direct the current pipeline's output to a particular location.
    ///
    /// See [`crate::OUTPUT`] to find out which outputs are available.
    pub fn request_manifest_output(out_type: u32) -> u32;

    /// Ask a particular capability to fill the `buffer` with input.
    ///
    /// Invalid parameters will trigger a trap and abort at runtime.
    pub fn request_provider_response(
        buffer: *mut u8,
        buffer_len: u32,
        capability_id: u32,
    ) -> u32;
}