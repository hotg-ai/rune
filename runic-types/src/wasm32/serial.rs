use crate::{wasm32::intrinsics, Sink, OUTPUT};

#[non_exhaustive]
pub struct Serial {}

impl Serial {
    pub fn new() -> Self {
        unsafe {
            intrinsics::request_manifest_output(OUTPUT::SERIAL as u32);
        }

        Serial {}
    }
}

impl<T> Sink<T> for Serial {
    fn consume(&mut self, _input: T) {
        // TODO: Wire up the VM to accept data for the serial connection. At
        // the moment we just log the output from a model to the debug console
        // so outputs aren't actually used.
    }
}
