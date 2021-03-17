use log::{Log, Metadata, Record, Level, LevelFilter};
use alloc::vec::Vec;
use core::{cell::RefCell, fmt::Write};
use super::intrinsics;

/// An implementation of [`Log`] which uses [`intrinsics::_debug()`] to send
/// log messages to the runtime.
#[derive(Debug, Clone)]
pub struct Logger {
    filter: LevelFilter,
}

impl Logger {
    pub const fn new() -> Self {
        if cfg!(debug_assertions) {
            Logger::with_level_filter(LevelFilter::Debug)
        } else {
            Logger::with_level_filter(LevelFilter::Info)
        }
    }

    pub const fn with_level_filter(filter: LevelFilter) -> Self {
        Logger {
            buffer: RefCell::new(Vec::with_capacity(1024)),
            filter,
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.filter
    }

    fn log(&self, r: &Record<'_>) {
        if !self.enabled(r.metadata()) {
            return;
        }

        let record = SerializableRecord {
            level: r.level(),
            message: r.args().to_string(),
            target: r.target(),
            module_path: r.module_path(),
            file: r.file(),
            line: r.line(),
        };

        match serde_json::to_string(record) {
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

#[derive(Debug, serde::Serialize)]
struct SerializableRecord<'a> {
    level: Level,
    message: &'a str,
    target: &'a str,
    module_path: Option<&'a str>,
    file: Option<&'a str>,
    line: Option<u32>,
}
