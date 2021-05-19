#[macro_use]
mod utils;

mod fft;
mod image_normalization;
mod noise_filtering;
mod normalize;

use self::{
    fft::Fft, image_normalization::ImageNormalization,
    noise_filtering::NoiseFiltering, normalize::Normalize,
};
use pyo3::{PyResult, Python, types::PyModule};

/// Bindings to proc blocks and Digital Signal Processing routines commonly used
/// during training.
pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Fft>()?;
    m.add_class::<Normalize>()?;
    m.add_class::<NoiseFiltering>()?;
    m.add_class::<ImageNormalization>()?;

    Ok(())
}
