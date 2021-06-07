use log::{Log, Metadata, Record};
use runic_types::{SerializableRecord, BufWriter};
use crate::intrinsics;
use alloc::borrow::Cow;
use core::fmt::{Display, Write};

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

        let mut buffer = [0; 4096];
        let message = match write_to_buffer(&mut buffer, r.args()) {
            Ok(m) => m,
            Err(_) => return,
        };

        let record = SerializableRecord {
            level: r.level(),
            message: Cow::Borrowed(message),
            target: r.target().into(),
            module_path: r.module_path().map(Cow::Borrowed),
            file: r.file().map(Cow::Borrowed),
            line: r.line(),
        };

        let mut json_buffer = [0; 4096];

        match serde_json_core::to_slice(&record, &mut json_buffer) {
            Ok(bytes_written) => unsafe {
                let payload = &json_buffer[..bytes_written];
                intrinsics::_debug(payload.as_ptr(), payload.len() as u32);
            },
            Err(_) => {
                // Oh well, we tried
                return;
            },
        }
    }

    fn flush(&self) {
        // nothing to do here
    }
}

fn write_to_buffer<'buf, D: Display>(
    buffer: &'buf mut [u8],
    item: &D,
) -> Result<&'buf str, core::fmt::Error> {
    let mut writer = BufWriter::new(buffer);
    write!(writer, "{}", item)?;

    core::str::from_utf8(writer.written())
        .map_err(|_| core::fmt::Error::default())
}
