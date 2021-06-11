use fft::ShortTimeFourierTransform;
use pyo3::{
    PyAny, PyObject, PyResult, Python, ToPyObject,
    prelude::{pyclass, pymethods},
};
use rune_pb_core::Transform;
use crate::proc_blocks::utils;

/// A Fast Fourier Transform.
#[pyclass(module = "rune_py")]
#[derive(Default, Clone, PartialEq)]
pub struct Fft {
    inner: ShortTimeFourierTransform,
}

#[pymethods]
impl Fft {
    #[new]
    pub fn new() -> Self { Fft::default() }

    #[call]
    pub fn call(&mut self, py: Python, iter: &PyAny) -> PyResult<PyObject> {
        let input = utils::to_tensor(iter)?;

        let spectrum = py.allow_threads(move || self.inner.transform(input));

        utils::to_numpy(py, &spectrum).map(|obj| obj.to_object(py))
    }
}

getters_and_setters! {
    impl Fft {
        inner.bins: usize;
        /// The frequency used to sample the audio data.
        inner.sample_rate: u32;
    }
}
