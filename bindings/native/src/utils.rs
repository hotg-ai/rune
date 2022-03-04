use std::{
    alloc::{GlobalAlloc, Layout},
    os::raw::c_char,
};

pub(crate) fn c_str(rust_str: &str) -> *mut c_char {
    unsafe {
        // Note: Use +1 and alloc_zeroed() to get the null terminator
        let layout = Layout::array::<u8>(rust_str.len() + 1).unwrap();
        let buffer = std::alloc::System.alloc_zeroed(layout);

        assert!(!buffer.is_null(), "Allocation failed");

        std::ptr::copy_nonoverlapping(
            rust_str.as_ptr(),
            buffer,
            rust_str.len(),
        );

        buffer.cast()
    }
}
