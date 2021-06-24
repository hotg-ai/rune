use pyo3::{
    PyAny, PyObject, PyResult, Python, ToPyObject,
    prelude::{pyclass, pymethods},
};

#[pyclass(module = "rune_py")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Normalize {}

#[pymethods]
impl Normalize {
    #[new]
    pub fn new() -> Self { Normalize {} }

    #[call]
    pub fn call(&self, py: Python, input: &PyAny) -> PyResult<PyObject> {
        let mut values: Vec<f64> = Vec::new();

        for value in input.iter()? {
            values.push(value?.extract()?);
        }

        ::normalize::normalize(&mut values);

        Ok(values.to_object(py))
    }
}
