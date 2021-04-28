use pyo3::{
    PyAny, PyObject, PyResult, Python, ToPyObject,
    prelude::{pyclass, pymethods},
};
use ::fft::Fft as UnderlyingFft;
use runic_types::{Tensor, Transform};

/// A Fast Fourier Transform.
#[pyclass(module = "proc_blocks.fft")]
#[derive(Default, Clone, PartialEq)]
pub struct Fft {
    inner: UnderlyingFft,
}

macro_rules! getters_and_setters {
    (impl $owner:ty { $( $(#[$meta:meta])* $property:ident : $type:ty ;)* }) => {
        $(
            paste::paste! {
                #[pymethods]
                impl $owner {
                    #[getter]
                    $( #[$meta] )*
                    pub fn $property(&self) -> $type { self.inner.$property() }

                    #[setter]
                    pub fn [< set_ $property >](&mut self, $property : $type) {
                        self.inner.[< set_ $property >]($property);
                    }
                }
            }
        )*
    };
}

#[pymethods]
impl Fft {
    #[new]
    pub fn new() -> Self { Fft::default() }

    #[call]
    pub fn call(&mut self, py: Python, iter: &PyAny) -> PyResult<PyObject> {
        let input = crate::utils::to_tensor(iter)?;

        let spectrum =
            py.allow_threads(move || self.inner.clone().transform(input));

        crate::utils::to_numpy(py, &spectrum).map(|obj| obj.to_object(py))
    }
}

getters_and_setters! {
    impl Fft {
        bins: usize;
        even_smoothing: f32;
        min_signal_remaining: f32;
        odd_smoothing: f32;
        offset: f32;
        /// The frequency used to sample the audio data.
        sample_rate: u32;
        smoothing_bits: u32;
        gain_bits: i32;
        strength: f32;
    }
}
