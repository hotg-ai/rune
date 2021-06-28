use std::convert::TryFrom;

use pyo3::{
    IntoPy, PyAny, PyErr, PyObject, PyObjectProtocol, PyRef, PyResult, Python,
    ToPyObject,
    basic::CompareOp,
    exceptions::{PyAssertionError, PyTypeError},
    prelude::{FromPyObject, pyclass, pymethods, pyproto},
};
use runic_types::Tensor;
use rune_proc_blocks::Transform;

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
    #[args("*", red = "None", green = "None", blue = "None")]
    pub fn new(
        red: Option<&PyAny>,
        green: Option<&PyAny>,
        blue: Option<&PyAny>,
    ) -> PyResult<ImageNormalization> {
        let mut inner = image_normalization::ImageNormalization::default();

        update_distribution(&mut inner.red, red)?;
        update_distribution(&mut inner.blue, blue)?;
        update_distribution(&mut inner.green, green)?;

        Ok(ImageNormalization { inner })
    }

    #[call]
    pub fn call(&mut self, py: Python, iter: &PyAny) -> PyResult<PyObject> {
        let input: Tensor<f32> = utils::to_tensor(iter)?;

        let normalized = py.allow_threads(move || self.inner.transform(input));

        utils::to_numpy(py, &normalized).map(|obj| obj.to_object(py))
    }

    #[getter]
    pub fn red(&self) -> Distribution { self.inner.red.into() }

    #[setter]
    pub fn set_red(&mut self, d: Distribution) { self.inner.red = d.into(); }

    #[getter]
    pub fn blue(&self) -> Distribution { self.inner.blue.into() }

    #[setter]
    pub fn set_blue(&mut self, d: Distribution) { self.inner.blue = d.into(); }

    #[getter]
    pub fn green(&self) -> Distribution { self.inner.green.into() }

    #[setter]
    pub fn set_green(&mut self, d: Distribution) {
        self.inner.green = d.into();
    }
}

#[pyproto]
impl PyObjectProtocol for ImageNormalization {
    fn __repr__(&self) -> String { format!("{:?}", self) }
}

fn update_distribution(
    dest: &mut image_normalization::Distribution,
    value: Option<&PyAny>,
) -> PyResult<()> {
    if let Some(value) = value {
        let distribution = Distribution::try_from(value)?;
        *dest = distribution.into();
    }

    Ok(())
}

/// A normal distribution.
#[pyclass]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Distribution {
    #[pyo3(get, set)]
    mean: f32,
    #[pyo3(get, set)]
    std_dev: f32,
}

#[pymethods]
impl Distribution {
    #[new]
    pub fn new(mean: f32, std_dev: f32) -> PyResult<Self> {
        if std_dev == 0.0 {
            return Err(PyAssertionError::new_err(
                "The standard deviation must be non-zero",
            ));
        }

        Ok(Distribution { mean, std_dev })
    }
}

#[pyproto]
impl PyObjectProtocol for Distribution {
    fn __repr__(&self) -> String { format!("{:?}", self) }

    fn __richcmp__(
        &'p self,
        other: PyRef<'p, Distribution>,
        op: CompareOp,
    ) -> PyResult<PyObject> {
        let py = other.py();

        match op {
            CompareOp::Eq => Ok((*self == *other).into_py(py)),
            CompareOp::Ne => Ok((*self != *other).into_py(py)),
            _ => Ok(py.NotImplemented()),
        }
    }
}

impl<'py> TryFrom<&'py PyAny> for Distribution {
    type Error = PyErr;

    fn try_from(ob: &'py PyAny) -> Result<Self, Self::Error> {
        if let Ok(d) = ob.extract() {
            return Ok(d);
        }

        if let Ok((mean, std_dev)) = ob.extract() {
            return Ok(Distribution { mean, std_dev });
        }

        if let Ok([mean, std_dev]) = ob.extract::<[f32; 2]>() {
            return Ok(Distribution { mean, std_dev });
        }

        #[derive(FromPyObject)]
        struct DictLike {
            #[pyo3(item)]
            mean: f32,
            #[pyo3(item)]
            std_dev: f32,
        }

        if let Ok(DictLike { mean, std_dev }) = ob.extract() {
            return Ok(Distribution { mean, std_dev });
        }

        Err(PyTypeError::new_err("Expected a 2-element tuple or a dict"))
    }
}

impl From<Distribution> for image_normalization::Distribution {
    fn from(d: Distribution) -> Self {
        image_normalization::Distribution::new(d.mean, d.std_dev)
    }
}

impl From<image_normalization::Distribution> for Distribution {
    fn from(d: image_normalization::Distribution) -> Self {
        Distribution {
            mean: d.mean,
            std_dev: d.standard_deviation,
        }
    }
}
