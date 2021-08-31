//! Functions provided by the host runtime.
//!
//! You probably shouldn't be touching things in here unless you know what you
//! are doing.

use core::marker::PhantomData;

/// A FFI-safe `&str`.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct StringRef<'a> {
    data: *const u8,
    len: u32,
    _lifetime: PhantomData<&'a str>,
}

impl<'a> From<&'a str> for StringRef<'a> {
    fn from(s: &'a str) -> StringRef<'a> {
        StringRef {
            data: s.as_ptr(),
            len: s.len() as u32,
            _lifetime: PhantomData,
        }
    }
}

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

    /// Load a model using a mimetype and a list of "descriptors" for the inputs
    /// and outputs.
    ///
    /// The mimetype will typically be something like
    /// `"application/tflite-model"` and is used to figure out what type of
    /// model is being provided.
    ///
    /// A tensor descriptor is a string like `"i16[3, 256, 256]"` which
    /// describes a tensor's shape (i.e. element type and dimensions).
    ///
    /// ```rust,no_run
    /// let inputs = [StringRef::from("i16[1920]"), StringRef::from("u8[3, 256, 256]")];
    /// let outputs = [StringRef::from("f32[1]")];
    /// let model = b"...";
    /// let mimetype = "application/tensorflow-lite";
    ///
    /// # unsafe {
    /// rune_model_load(
    ///    mimetype.as_ptr(),
    ///    mimetype.len() as u32,
    ///    model.as_ptr(),
    ///    model.len() as u32,
    ///    inputs.as_ptr(),
    ///    inputs.len() as u32,
    ///    outputs.as_ptr(),
    ///    outputs.len() as u32,
    /// );
    /// # }
    /// ```
    pub fn rune_model_load(
        mimetype: *const u8,
        mimetype_len: u32,
        model: *const u8,
        model_len: u32,
        input_descriptors: *const StringRef<'_>,
        input_len: u32,
        output_descriptors: *const StringRef<'_>,
        output_len: u32,
    ) -> u32;

    /// Run inference using a model.
    ///
    /// The model's output will be written to the `output` buffers.
    ///
    /// Model failures will trigger a trap and abort at runtime.
    pub fn rune_model_infer(
        model_id: u32,
        inputs: *const *const u8,
        outputs: *mut *mut u8,
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
    /// See [`hotg_rune_core::capabilities`] to find out which capabilities are
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
    /// See [`hotg_rune_core::outputs`] to find out which outputs are available.
    /// Invalid or unsupported parameters will trigger a trap and abort at
    /// runtime.
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

    /// Open a named resource, returning a unique ID that can be used to .
    ///
    /// Invalid parameters will trigger a trap and abort at runtime.
    pub fn rune_resource_open(name: *const u8, name_len: u32) -> u32;

    /// Read data from a resource into the provided buffer.
    ///
    /// Invalid parameters will trigger a trap and abort at runtime.
    pub fn rune_resource_read(
        resource_id: u32,
        buffer: *mut u8,
        buffer_len: u32,
    ) -> u32;

    /// Close a resource
    ///
    /// Invalid parameters will trigger a trap and abort at runtime.
    pub fn rune_resource_close(resource_id: u32);
}
