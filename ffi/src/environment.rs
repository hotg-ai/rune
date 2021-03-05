use std::{ffi::c_void, os::raw::{c_char, c_int}};

use anyhow::Error;
use rune_runtime::NotSupportedError;

/// A vtable providing the Rune with functions for interacting with its
/// environment.
///
/// # Safety
///
/// All methods must be thread-safe and manage its own synchronisation.
///
/// The `user_data` object must also be safe to move between threads.
#[repr(C)]
pub struct Environment {
    /// Arbitrary user data that can be used to persist state between method
    /// calls.
    pub user_data: *mut c_void,
    /// Log a message.
    pub log: Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_int)>,
    /// Fill the provided buffer with random data.
    ///
    /// A negative return value indicates an error has occurred. Non-negative
    /// values indicate success and the number of bytes written.
    pub fill_random:
        Option<unsafe extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int>,
    /// Fill the provided buffer with accelerometer data.
    ///
    /// Each sample consists of three 32-bit floating point values, with the
    /// integer argument specifying how many samples the buffer can hold.
    ///
    /// A negative return value indicates an error has occurred. Non-negative
    /// values indicate success and the number of samples written.
    pub fill_accelerometer:
        Option<unsafe extern "C" fn(*mut c_void, *mut f32, c_int) -> c_int>,
    /// Render an image to the provided buffer with the specified height and
    /// width.
    ///
    /// A negative return value indicates an error has occurred. Non-negative
    /// values indicate success and the number of bytes written.
    pub fill_image: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut c_char,
            c_int,
            c_int,
            c_int,
        ) -> c_int,
    >,
    /// A destructor for the cleaning up the user data when the module is
    /// unloaded.
    pub user_data_free: Option<unsafe extern "C" fn(*mut c_void)>,
}

impl rune_runtime::Environment for Environment {
    fn log(&self, msg: &str) {
        if let Some(log) = self.log {
            unsafe {
                log(self.user_data, msg.as_ptr().cast(), msg.len() as c_int);
            }
        }
    }

    fn fill_random(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        let fill_random = self.fill_random.ok_or(NotSupportedError)?;

        unsafe {
            let bytes_written = fill_random(
                self.user_data,
                buffer.as_mut_ptr().cast(),
                buffer.len() as c_int,
            );

            if bytes_written >= 0 {
                Ok(bytes_written as usize)
            } else {
                Err(Error::msg("Call failed"))
            }
        }
    }

    fn fill_accelerometer(
        &self,
        buffer: &mut [[f32; 3]],
    ) -> Result<usize, Error> {
        let fill_accelerometer =
            self.fill_accelerometer.ok_or(NotSupportedError)?;

        unsafe {
            let bytes_written = fill_accelerometer(
                self.user_data,
                buffer.as_mut_ptr().cast(),
                buffer.len() as c_int,
            );

            if bytes_written >= 0 {
                Ok(bytes_written as usize)
            } else {
                Err(Error::msg("Call failed"))
            }
        }
    }

    fn fill_image(
        &self,
        buffer: &mut [u8],
        height: u32,
        width: u32,
    ) -> Result<usize, Error> {
        let fill_image = self.fill_image.ok_or(NotSupportedError)?;

        unsafe {
            let bytes_written = fill_image(
                self.user_data,
                buffer.as_mut_ptr().cast(),
                buffer.len() as c_int,
                height as c_int,
                width as c_int,
            );

            if bytes_written >= 0 {
                Ok(bytes_written as usize)
            } else {
                Err(Error::msg("Call failed"))
            }
        }
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        if let Some(free) = self.user_data_free {
            unsafe {
                free(self.user_data);
            }
        }
    }
}

// Safety: Upheld by the caller.
unsafe impl Send for Environment {}
unsafe impl Sync for Environment {}
