pub mod alloc;
mod capability;
mod guards;
pub mod intrinsics;
mod logging;
mod model;
pub mod serial;
mod stats_allocator;

pub use self::{
    capability::{GenericCapability, Accelerometer, Random, Sound, Raw, Image},
    guards::{SetupGuard, PipelineGuard},
    logging::Logger,
    model::Model,
    serial::Serial,
};

use core::{alloc::Layout, fmt::Write, panic::PanicInfo};
use crate::{BufWriter, wasm32::alloc::Allocator};
use dlmalloc::GlobalDlmalloc;

#[global_allocator]
pub static ALLOCATOR: Allocator<GlobalDlmalloc> =
    Allocator::new(GlobalDlmalloc);

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    static mut PANICKING: bool = false;

    unsafe {
        // We need to guard against the possiblity that logging a panic may
        // in turn trigger a panic (e.g. due to OOM), causing infinite
        // recursion.
        if !PANICKING {
            PANICKING = true;

            // First we try to log the panic at the ERROR level. This should
            // be translated into a runtime trap, so under most circumstances
            // the log call won't return and our user will get a nice error
            // message.
            log::error!("{}", info);
        }

        // However, some times the runtime won't receive the log message (e.g.
        // log level filtering or because an OOM in logging recursively
        // triggered the panic handler). If that is the case, we still try to
        // send *some* message to the runtime so they know the world is broken.

        // Safety: We need our own buffer for panic messages in case the
        // allocator is fubar. Runes are single-threaded, so we can
        // guarantee we'll never have aliased mutation.
        static mut DEBUG_BUFFER: [u8; 1024] = [0; 1024];
        let mut w = BufWriter::new(&mut DEBUG_BUFFER);

        if write!(w, "{}", info).is_ok() {
            let written = w.written();
            intrinsics::_debug(written.as_ptr(), written.len() as u32);
        }

        // And now we've done everything we can, we ungracefully crash.
        core::arch::wasm32::unreachable()
    }
}

#[alloc_error_handler]
fn on_alloc_error(layout: Layout) -> ! {
    panic!(
        "memory allocation of {} bytes failed ({:?})",
        layout.size(),
        ALLOCATOR.stats()
    );
}
