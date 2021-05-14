#[macro_use]
mod utils;

mod fft;
mod noise_filtering;
mod normalize;

use self::fft::Fft;
use self::normalize::Normalize;
use self::noise_filtering::NoiseFiltering;
use pyo3::{PyResult, Python, types::PyModule};

/// Bindings to proc blocks and Digital Signal Processing routines commonly used
/// during training.
pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Fft>()?;
    m.add_class::<Normalize>()?;
    m.add_class::<NoiseFiltering>()?;

    Ok(())
}
