use pyo3::{
    PyAny, PyObject, PyResult, Python, ToPyObject,
    prelude::{pyclass, pymethods},
};
use runic_types::Transform;
use crate::proc_blocks::utils;

#[pyclass(module = "rune_py")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NoiseFiltering {
    inner: noise_filtering::NoiseFiltering,
}

#[pymethods]
impl NoiseFiltering {
    #[new]
    pub fn new() -> Self { NoiseFiltering::default() }

    #[call]
    pub fn call(&mut self, py: Python, iter: &PyAny) -> PyResult<PyObject> {
        let input = utils::to_tensor(iter)?;

        let spectrum = py.allow_threads(move || self.inner.transform(input));

        utils::to_numpy(py, &spectrum).map(|obj| obj.to_object(py))
    }
}

getters_and_setters! {
    impl NoiseFiltering {
        inner.even_smoothing: f32;
        inner.min_signal_remaining: f32;
        inner.odd_smoothing: f32;
        inner.offset: f32;
        /// The frequency used to sample the audio data.
        inner.smoothing_bits: u32;
        inner.gain_bits: i32;
        inner.strength: f32;
    }
}
