use pyo3::{
    PyAny, PyObject, PyObjectProtocol, PyResult, Python, ToPyObject,
    prelude::{pyclass, pymethods, pyproto},
};
use rune_core::Tensor;
use rune_proc_blocks::Transform;

use crate::proc_blocks::utils;

/// A proc block which will normalize each channel of an image by scaling each
/// pixel value to the range `[0, 1]`.
#[pyclass]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageNormalization {
    inner: image_normalization::ImageNormalization,
}

#[pymethods]
impl ImageNormalization {
    #[new]
    pub fn new() -> ImageNormalization {
        ImageNormalization {
            inner: image_normalization::ImageNormalization::default(),
        }
    }

    #[call]
    pub fn call(&mut self, py: Python, iter: &PyAny) -> PyResult<PyObject> {
        let input: Tensor<f32> = utils::to_tensor(iter)?;

        let normalized = py.allow_threads(move || self.inner.transform(input));

        utils::to_numpy(py, &normalized).map(|obj| obj.to_object(py))
    }
}

#[pyproto]
impl PyObjectProtocol for ImageNormalization {
    fn __repr__(&self) -> String { format!("{:?}", self) }
}
