use std::{
    fmt::{self, Debug, Formatter},
    os::raw::{c_char, c_int, c_void},
};

use anyhow::{Context, Error};

#[repr(C)]
pub struct Output {
    /// A pointer to some state that will be passed to any of the other
    /// output methods.
    pub user_data: *mut c_void,
    /// Consume the provided data.
    pub consume: Option<
        unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_int,
    >,
    /// Free the `user_data`.
    pub destroy: Option<unsafe extern "C" fn(*mut c_void)>,
}

impl rune_runtime::outputs::Output for Output {
    fn consume(&mut self, buffer: &[u8]) -> Result<(), Error> {
        let consume = self.consume.context("Not Supported")?;

        unsafe {
            if consume(
                self.user_data,
                buffer.as_ptr().cast(),
                buffer.len() as c_int,
            ) >= 0
            {
                Ok(())
            } else {
                Err(Error::msg(
                    "The underlying implementation returned an error",
                ))
            }
        }
    }
}

impl Debug for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Output {
            user_data,
            consume,
            destroy,
        } = self;

        f.debug_struct("Output")
            .field("user_data", &format_args!("{:p}", user_data))
            .field("consume", &format_args!("{:p}", consume))
            .field("destroy", &format_args!("{:p}", destroy))
            .finish()
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        if let Some(free) = self.destroy {
            unsafe {
                free(self.user_data);
            }
        }
    }
}

unsafe impl Send for Output {}
unsafe impl Sync for Output {}
