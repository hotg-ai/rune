mod accelerometer;
pub mod alloc;
mod debug;
mod guards;
mod image;
pub mod intrinsics;
mod logging;
mod model;
mod random;
mod serial;
mod sound;

pub use accelerometer::Accelerometer;
pub use image::Image;
pub use guards::{SetupGuard, PipelineGuard};
pub use model::Model;
pub use random::Random;
pub use serial::Serial;
pub use sound::Sound;
pub use logging::Logger;

use core::{alloc::Layout, fmt::Write, panic::PanicInfo};
use debug::BufWriter;
use wee_alloc::WeeAlloc;
use crate::{Buffer, Value};

#[global_allocator]
pub static ALLOCATOR: self::alloc::StatsAllocator<
    self::alloc::DebugAllocator<WeeAlloc<'static>>,
> = self::alloc::StatsAllocator::new(self::alloc::DebugAllocator::new(
    WeeAlloc::INIT,
));

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    // Logging at the error level should result in a runtime trap. This way
    // we get both a nice error message.
    log::error!("{}", info);

    // However, if we couldn't send the error log to the runtime (e.g. due to
    // memory issues), we should still try to generate *some* error message
    // and abort.

    // Safety: We need our own buffer for panic messages in case the allocator
    // is FUBAR. Runes are single-threaded, so we can guarantee we'll never
    // have aliased mutation.
    unsafe {
        static mut DEBUG_BUFFER: [u8; 1024] = [0; 1024];
        let mut w = BufWriter::new(&mut DEBUG_BUFFER);

        if write!(w, "{}", info).is_ok() {
            w.flush();
        }

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
