use super::{ALLOCATOR, stats_allocator::Stats, Logger};

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
/// letting `rune-core` run code at the start and end.
#[derive(Debug)]
pub struct SetupGuard {
    _log: AllocationLogger,
}

impl SetupGuard {
    pub fn new() -> Self {
        static LOGGER: Logger = Logger::new();
        log::set_max_level(log::STATIC_MAX_LEVEL);
        log::set_logger(&LOGGER).unwrap();

        log::debug!("Initializing");

        SetupGuard {
            _log: AllocationLogger::new("Setup", ALLOCATOR.stats()),
        }
    }
}

impl Default for SetupGuard {
    fn default() -> Self { SetupGuard::new() }
}

impl Drop for SetupGuard {
    fn drop(&mut self) {
        log::debug!("Initialization complete");
    }
}

/// A guard type which should be alive for the duration of a single pipeline
/// run, letting `rune-core` run code as necessary.
#[derive(Debug)]
pub struct PipelineGuard {
    _log: AllocationLogger,
}

impl PipelineGuard {
    pub fn new() -> Self {
        log::debug!("Running the pipeline");

        PipelineGuard {
            _log: AllocationLogger::new("Pipeline", ALLOCATOR.stats()),
        }
    }
}

impl Default for PipelineGuard {
    fn default() -> Self { PipelineGuard::new() }
}

impl Drop for PipelineGuard {
    fn drop(&mut self) {
        log::debug!("Pipeline finished");
    }
}
