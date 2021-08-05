#[cfg(feature = "builtins")]
pub mod common_outputs;

use anyhow::Error;
use std::fmt::{self, Debug, Display, Formatter};
use hotg_rune_core::{InvalidConversionError, Value};

/// A primitive type that can be passed between host and WebAssembly guest.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum WasmValue {
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
}

impl Display for WasmValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            WasmValue::F32(float) => write!(f, "{}_f32", float),
            WasmValue::F64(double) => write!(f, "{}_f64", double),
            WasmValue::I32(int) => write!(f, "{}_i32", int),
            WasmValue::I64(long) => write!(f, "{}_i64", long),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum WasmType {
    F32,
    F64,
    I32,
    I64,
}

pub trait Image<Registrar> {
    fn initialize_imports(self, registrar: &mut Registrar);
}

/// Something a Rune can send output to.
pub trait Output: Send + Debug + 'static {
    fn consume(&mut self, buffer: &[u8]) -> Result<(), Error>;
}

pub trait Capability: Send + Debug + 'static {
    /// Generate the desired input, writing it to the provided buffer and
    /// returning the number of bytes written.
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error>;

    fn set_parameter(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), ParameterError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ParameterError {
    #[error("The parameter isn't supported")]
    UnsupportedParameter,
    #[error("{:?} is an invalid value", value)]
    InvalidValue {
        value: Value,
        #[source]
        reason: Error,
    },
    #[error("{}", _0)]
    IncorrectType(InvalidConversionError),
}

impl ParameterError {
    pub fn invalid_value<E>(value: Value, reason: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ParameterError::InvalidValue {
            value,
            reason: reason.into(),
        }
    }
}
