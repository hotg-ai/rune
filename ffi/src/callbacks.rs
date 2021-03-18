use std::{
    ffi::c_void,
    os::raw::{c_char, c_int},
};

use crate::{Output, capability::Capability};

/// A vtable providing the Rune with functions for interacting with its
/// environment.
///
/// # Safety
///
/// All methods must be thread-safe and manage its own synchronisation.
///
/// The `user_data` object must also be safe to move between threads.
#[repr(C)]
pub struct Callbacks {
    /// Arbitrary user data that can be used to persist state between method
    /// calls.
    pub user_data: *mut c_void,
    /// Log a message.
    pub log: Option<
        unsafe extern "C" fn(
            *mut c_void,
            c_int,
            *const c_char,
            c_int,
            *const c_char,
            c_int,
        ),
    >,
    /// Initialize a capability vtable which produces random data.
    pub random:
        Option<unsafe extern "C" fn(*mut c_void, *mut Capability) -> c_int>,
    /// Initialize a capability vtable which yields accelerometer samples.
    ///
    /// Each sample consists of three 32-bit floating point values, with the
    /// integer argument specifying how many samples the buffer can hold.
    pub accelerometer:
        Option<unsafe extern "C" fn(*mut c_void, *mut Capability) -> c_int>,
    /// Initialize a capability vtable which yields images.
    pub image:
        Option<unsafe extern "C" fn(*mut c_void, *mut Capability) -> c_int>,
    /// Initialize the serial output vtable.
    pub serial: Option<unsafe extern "C" fn(*mut c_void, *mut Output) -> c_int>,
    /// A destructor for the cleaning up the user data when the module is
    /// unloaded.
    pub destroy: Option<unsafe extern "C" fn(*mut c_void)>,
}

impl Drop for Callbacks {
    fn drop(&mut self) {
        if let Some(free) = self.destroy {
            unsafe {
                free(self.user_data);
            }
        }
    }
}

// Safety: Upheld by the caller.
unsafe impl Send for Callbacks {}
unsafe impl Sync for Callbacks {}
