mod accelerometer;
mod image;
pub mod intrinsics;
mod model;
mod random;
mod serial;

#[doc(hidden)] // only exposed so we can refer to the buffer and writer
#[macro_use]
pub mod debug;

pub use model::Model;
pub use random::Random;
pub use serial::Serial;
pub use accelerometer::Accelerometer;
pub use image::Image;

use core::{alloc::Layout, fmt::Write, panic::PanicInfo};
use debug::BufWriter;
use wee_alloc::WeeAlloc;
use crate::Buffer;

#[global_allocator]
pub static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    unsafe {
        let mut buffer = [0; 512];
        let mut w = BufWriter::new(&mut buffer);

        if write!(w, "{}", info).is_ok() {
            w.flush();
        }

        core::arch::wasm32::unreachable()
    }
}

#[alloc_error_handler]
fn on_alloc_error(layout: Layout) -> ! {
    panic!("memory allocation of {} bytes failed", layout.size())
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
