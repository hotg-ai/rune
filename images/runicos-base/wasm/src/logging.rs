use alloc::borrow::Cow;
use core::fmt::{Display, Write};

use hotg_rune_core::SerializableRecord;
use log::{Log, Metadata, Record};

use crate::{intrinsics, BufWriter};

/// An implementation of [`Log`] which uses [`intrinsics::_debug()`] to send
/// log messages to the runtime.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Logger {}

impl Logger {
    pub const fn new() -> Self { Logger {} }
}

fn write(msg: &[u8]) {
    unsafe {
        intrinsics::_debug(msg.as_ptr(), msg.len() as u32);
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool { true }

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
            Ok(bytes_written) => write(&json_buffer[..bytes_written]),
            Err(_) => {
                // Oh well, we tried
                write(b"Unable to serialize a log message as JSON");
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
