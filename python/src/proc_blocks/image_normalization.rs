use pyo3::{
    PyAny, PyObject, PyResult, Python, ToPyObject,
    prelude::{pyclass, pymethods},
};
use runic_types::{Tensor, Transform};

use crate::proc_blocks::utils;

/// A proc block which will normalize each channel of an image by fitting each
/// channel to a standard normal distribution.
#[pyclass]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageNormalization {
    inner: image_normalization::ImageNormalization,
}

#[pymethods]
impl ImageNormalization {
    #[new]
    pub fn new() -> ImageNormalization { ImageNormalization::default() }

    #[call]
    pub fn call(&mut self, py: Python, iter: &PyAny) -> PyResult<PyObject> {
        let input: Tensor<f32> = utils::to_tensor(iter)?;

        let spectrum = py.allow_threads(move || self.inner.transform(input));

        utils::to_numpy(py, &spectrum).map(|obj| obj.to_object(py))
    }
}
