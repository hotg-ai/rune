pub mod fft;

use crate::fft::Fft;
use pyo3::{PyResult, Python, prelude::pymodule, types::PyModule};

/// Bindings to common proc blocks used in Rune.
#[pymodule]
fn proc_blocks(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Fft>()?;

    Ok(())
}
