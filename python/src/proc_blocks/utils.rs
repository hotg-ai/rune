use pyo3::{PyAny, PyResult, Python, FromPyObject};
use runic_types::Tensor;
use numpy::{Element, PyArrayDyn, ToPyArray};

pub(crate) fn to_tensor<T>(object: &PyAny) -> PyResult<Tensor<T>>
where
    T: Element + for<'a> FromPyObject<'a>,
{
    if let Some(array) = object.cast_as::<PyArrayDyn<T>>().ok() {
        if let Ok(tensor) = numpy_to_tensor(array) {
            return Ok(tensor);
        }
    }

    // Assume it's a 1D vector.
    let mut input = Vec::new();

    for value in object.iter()? {
        let value: T = value?.extract()?;
        input.push(value);
    }

    Ok(Tensor::new_vector(input))
}

fn numpy_to_tensor<T>(array: &PyArrayDyn<T>) -> PyResult<Tensor<T>>
where
    T: Element,
{
    let array = array.readonly();
    let shape = array.shape();

    let elements = array.as_slice()?;

    Ok(Tensor::new_row_major(elements.into(), shape.to_vec()))
}

pub(crate) fn to_numpy<'py, T>(
    py: Python<'py>,
    tensor: &Tensor<T>,
) -> PyResult<&'py PyArrayDyn<T>>
where
    T: Element,
{
    let shape = tensor.dimensions();
    let elements = tensor.elements();

    elements.to_pyarray(py).reshape(shape)
}

macro_rules! getters_and_setters {
    (impl $owner:ty { $( $(#[$meta:meta])* $component:ident . $property:ident : $type:ty ;)* }) => {
        $(
            paste::paste! {
                #[pymethods]
                impl $owner {
                    #[getter]
                    $( #[$meta] )*
                    pub fn $property(&self) -> $type { self.$component.$property().clone() }

                    #[setter]
                    pub fn [< set_ $property >](&mut self, $property : $type) {
                        self.$component.[< set_ $property >]($property);
                    }
                }
            }
        )*
    };
}
