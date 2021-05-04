mod fft;
mod normalize;
mod utils;

use self::fft::Fft;
use self::normalize::Normalize;
use pyo3::{PyResult, Python, types::PyModule};

/// Bindings to proc blocks and Digital Signal Processing routines commonly used
/// during training.
pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Fft>()?;
    m.add_class::<Normalize>()?;

    Ok(())
}
