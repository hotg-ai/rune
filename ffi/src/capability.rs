use std::{
    fmt::{self, Debug, Formatter},
    os::raw::{c_char, c_int, c_void},
};
use anyhow::Error;
use rune_wasmer_runtime::capability::ParameterError;
use runic_types::{Type, Value};

/// An object which can be used to pass input from the outside world into the
/// Rune.
///
/// # Safety
///
/// The internal implementation must be thread-safe.
#[repr(C)]
pub struct Capability {
    /// A pointer to some state that will be passed to any of the other
    /// capability methods.
    pub user_data: *mut c_void,
    /// Fill the provided buffer with data.
    pub generate:
        Option<unsafe extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int>,
    /// Set a capability parameter.
    ///
    /// The parameters are:
    ///
    /// - A pointer to `user_data`
    /// - A pointer to the parameter name as a UTF-8 string
    /// - The parameter name's length in bytes
    /// - A pointer to the value encoded (little-endian)
    /// - The value's length in bytes
    /// - The value's type
    pub set_parameter: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const c_char,
            c_int,
            *const c_char,
            c_int,
            Type,
        ) -> c_int,
    >,
    /// Free the `user_data`.
    pub destroy: Option<unsafe extern "C" fn(*mut c_void)>,
}

impl rune_wasmer_runtime::capability::Capability for Capability {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let gen = self.generate.ok_or(ParameterError::UnsupportedParameter)?;

        unsafe {
            let bytes_written = gen(
                self.user_data,
                buffer.as_mut_ptr().cast(),
                buffer.len() as i32,
            );

            if bytes_written >= 0 {
                Ok(bytes_written as usize)
            } else {
                Err(Error::msg(
                    "The foreign function was unable to generate data",
                ))
            }
        }
    }

    fn set_parameter(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), ParameterError> {
        let set_parameter = self
            .set_parameter
            .ok_or(ParameterError::UnsupportedParameter)?;

        let mut buffer = Value::buffer();
        let bytes_written = value.to_le_bytes(&mut buffer);
        let ty = value.ty();

        unsafe {
            set_parameter(
                self.user_data,
                name.as_ptr().cast(),
                name.len() as c_int,
                buffer.as_ptr().cast(),
                bytes_written as c_int,
                ty,
            );
        }

        Ok(())
    }
}

impl Debug for Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Capability {
            user_data,
            generate,
            set_parameter,
            destroy,
        } = self;

        f.debug_struct("Capability")
            .field("user_data", &format_args!("{:p}", user_data))
            .field("generate", &format_args!("{:p}", generate))
            .field("set_parameter", &format_args!("{:p}", set_parameter))
            .field("destroy", &format_args!("{:p}", destroy))
            .finish()
    }
}

impl Drop for Capability {
    fn drop(&mut self) {
        if let Some(free) = self.destroy {
            unsafe {
                free(self.user_data);
            }
        }
    }
}

unsafe impl Send for Capability {}
unsafe impl Sync for Capability {}
