mod proc_blocks;

use pyo3::{PyResult, Python, prelude::pymodule, types::PyModule};

/// Bindings to the Rune project.
#[pymodule]
fn rune_py(py: Python, m: &PyModule) -> PyResult<()> {
    proc_blocks::register(py, m)?;

    Ok(())
}
