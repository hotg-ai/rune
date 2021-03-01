use crate::{wasm32::intrinsics, AsParamType, Transform};
use core::marker::PhantomData;

/// A machine learning model.
pub struct Model<Input, Output> {
    /// A unique identifier we can use to refer to the model.
    // FIXME: Change the VM to allow multiple models.
    _index: u32,
    _type: PhantomData<fn(Input) -> Output>,
}

impl<In, Out, const M: usize, const N: usize> Model<[In; M], [Out; N]> {
    /// Loads a model into the VM.
    pub fn load(raw_blob: &[u8]) -> Self {
        unsafe {
            let ix = intrinsics::tfm_preload_model(
                raw_blob.as_ptr(),
                raw_blob.len() as u32,
                M as u32,
                N as u32,
            );

            Model {
                _index: ix,
                _type: PhantomData,
            }
        }
    }
}

impl<In, Out, const M: usize, const N: usize> Transform<[In; M]>
    for Model<[In; M], [Out; N]>
where
    Out: AsParamType,
{
    type Output = [Out; N];

    fn transform(&mut self, input: [In; M]) -> [Out; N] {
        unsafe {
            let ptr = input.as_ptr() as *const u8;
            let length = core::mem::size_of_val(&input);

            // FIXME(Michael-F-Bryan): Figure out how we get the result back
            // from the VM. Ideally we should provide a buffer that model
            // outputs can be written to.
            let _got = intrinsics::tfm_model_invoke(ptr, length as u32);

            Out::zeroed_array()
        }
    }
}
