use super::{
    ALLOCATOR,
    stats_allocator::Stats,
    Logger,
};

#[derive(Debug, Clone, PartialEq)]
struct AllocationLogger {
    label: &'static str,
    initial: Stats,
}

impl AllocationLogger {
    const fn new(label: &'static str, initial_stats: Stats) -> Self {
        AllocationLogger {
            label,
            initial: initial_stats,
        }
    }
}

impl Drop for AllocationLogger {
    fn drop(&mut self) {
        let current = ALLOCATOR.stats();
        let delta = current - self.initial;
        log::debug!("{} {:?}", self.label, delta);
    }
}

/// A guard type which should be alive for the duration of the setup process,
/// letting `runic-types` run code at the start and end.
#[derive(Debug)]
pub struct SetupGuard {
    log: AllocationLogger,
}

impl SetupGuard {
    pub fn new() -> Self {
        static LOGGER: Logger = Logger::new();

        // Safety: The runtime won't try to change Rune memory while Rune code
        // is running, and Runes are single-threaded so there is no risk of
        // aliased mutation. The `MAX_LOG_LEVEL` docs also state that if
        // modifying it from the outside, the variable should always have a
        // valid value.
        log::set_max_level(unsafe { super::MAX_LOG_LEVEL });
        log::set_logger(&LOGGER).unwrap();

        SetupGuard {
            log: AllocationLogger::new("Setup", ALLOCATOR.stats()),
        }
    }
}

impl Default for SetupGuard {
    fn default() -> Self { SetupGuard::new() }
}

/// A guard type which should be alive for the duration of a single pipeline
/// run, letting `runic-types` run code as necessary.
#[derive(Debug)]
pub struct PipelineGuard {
    log: AllocationLogger,
}

impl PipelineGuard {
    pub fn new() -> Self {
        PipelineGuard {
            log: AllocationLogger::new("Pipeline", ALLOCATOR.stats()),
        }
    }
}

impl Default for PipelineGuard {
    fn default() -> Self { PipelineGuard::new() }
}
