use log::{Log, Metadata, Record};
use alloc::string::ToString;
use crate::{SerializableRecord, wasm32::intrinsics};

/// An implementation of [`Log`] which uses [`intrinsics::_debug()`] to send
/// log messages to the runtime.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Logger {}

impl Logger {
    pub const fn new() -> Self { Logger {} }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, r: &Record<'_>) {
        if !self.enabled(r.metadata()) {
            return;
        }

        let message = r.args().to_string();

        let record = SerializableRecord {
            level: r.level(),
            message: message.into(),
            target: r.target().into(),
            module_path: r.module_path().map(Into::into),
            file: r.file().map(Into::into),
            line: r.line(),
        };

        match serde_json::to_string(&record) {
            Ok(buffer) => unsafe {
                intrinsics::_debug(buffer.as_ptr(), buffer.len() as u32);
            },
            Err(_) => {
                // Oh well, we tried
            },
        }
    }

    fn flush(&self) {
        // nothing to do here
    }
}
