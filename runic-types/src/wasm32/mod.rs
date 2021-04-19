mod accelerometer;
pub mod alloc;
mod guards;
mod image;
pub mod intrinsics;
mod logging;
mod model;
mod random;
mod raw;
mod serial;
mod sound;
mod stats_allocator;

pub use accelerometer::Accelerometer;
pub use image::Image;
pub use guards::{SetupGuard, PipelineGuard};
pub use model::Model;
pub use random::Random;
pub use serial::Serial;
pub use sound::Sound;
pub use logging::Logger;

use core::{alloc::Layout, fmt::Write, panic::PanicInfo};
use crate::{Buffer, Value, BufWriter};
use self::alloc::Allocator;
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

fn copy_capability_data_to_buffer<B>(capability_id: u32, buffer: &mut B)
where
    B: Buffer,
{
    let byte_length = buffer.size_in_bytes() as u32;

    unsafe {
        let response_size = intrinsics::request_provider_response(
            buffer.as_mut_ptr() as _,
            byte_length,
            capability_id,
        );

        debug_assert_eq!(response_size, byte_length);
    }
}

fn set_capability_parameter(capability_id: u32, key: &str, value: Value) {
    unsafe {
        let mut buffer = Value::buffer();
        let bytes_written = value.to_le_bytes(&mut buffer);

        intrinsics::request_capability_set_param(
            capability_id,
            key.as_ptr(),
            key.len() as u32,
            buffer.as_ptr(),
            bytes_written as u32,
            value.ty().into(),
        );
    }
}
