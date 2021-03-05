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
