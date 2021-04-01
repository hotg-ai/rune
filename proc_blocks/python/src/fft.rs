use pyo3::{
    PyAny, PyObject, PyResult, Python, ToPyObject,
    prelude::{pyclass, pymethods},
};
use ::fft::Fft as UnderlyingFft;
use runic_types::Transform;

/// A Fast Fourier Transform.
#[pyclass(module = "proc_blocks.fft")]
#[derive(Default, Clone, PartialEq)]
pub struct Fft {
    inner: UnderlyingFft,
}

#[pymethods]
impl Fft {
    #[new]
    pub fn new(sample_rate: u32) -> Self {
        Fft {
            inner: UnderlyingFft::new().with_sample_rate(sample_rate),
        }
    }

    #[getter]
    pub fn sample_rate(&self) -> u32 { self.inner.sample_rate }

    #[setter]
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.inner.sample_rate = sample_rate;
    }

    #[call]
    pub fn call(&mut self, py: Python, iter: &PyAny) -> PyResult<PyObject> {
        let mut input = Vec::new();

        for value in iter.iter()? {
            let value: i16 = value?.extract()?;
            input.push(value);
        }

        let spectrum = self.inner.clone().transform(input.as_slice());

        Ok(spectrum.to_object(py))
    }
}
