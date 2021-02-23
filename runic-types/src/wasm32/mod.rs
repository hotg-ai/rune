pub mod intrinsics;
mod model;

#[doc(hidden)] // only exposed so we can refer to the buffer and writer
#[macro_use]
pub mod debug;

pub use model::Model;

use core::{alloc::Layout, panic::PanicInfo};
use wee_alloc::WeeAlloc;

#[global_allocator]
pub static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    debug!("Panic {}", info);

    unsafe { core::arch::wasm32::unreachable() }
}

#[alloc_error_handler]
fn on_alloc_error(layout: Layout) -> ! {
    panic!("memory allocation of {} bytes failed", layout.size())
}
