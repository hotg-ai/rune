mod proc_blocks;

use pyo3::{PyResult, Python, prelude::pymodule, types::PyModule};

/// Bindings to the Rune project.
#[pymodule]
fn rune_py(py: Python, m: &PyModule) -> PyResult<()> {
    proc_blocks::register(py, m)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__authors__", env!("CARGO_PKG_AUTHORS"))?;
    m.add("__license__", env!("CARGO_PKG_LICENSE"))?;

    Ok(())
}
