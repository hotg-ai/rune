mod fft;
mod normalize;
mod utils;

use crate::fft::Fft;
use crate::normalize::Normalize;
use pyo3::{PyResult, Python, prelude::pymodule, types::PyModule};

/// Bindings to common proc blocks used in Rune.
#[pymodule]
fn proc_blocks(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Fft>()?;
    m.add_class::<Normalize>()?;

    Ok(())
}
