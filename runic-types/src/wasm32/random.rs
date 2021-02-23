use crate::{wasm32::intrinsics, PipelineContext, Source, CAPABILITY};
use core::marker::PhantomData;

pub struct Random<T, const N: usize> {
    index: u32,
    _type: PhantomData<fn() -> [T; N]>,
}

impl<T, const N: usize> Random<T, N> {
    pub fn new() -> Self {
        unsafe {
            let index = intrinsics::request_capability(CAPABILITY::RAND as u32);

            Random {
                index,
                _type: PhantomData,
            }
        }
    }
}

impl<T, const N: usize> Source for Random<T, N>
where
    [T; N]: Default,
{
    type Output = [T; N];

    fn generate(&mut self, _ctx: &mut PipelineContext) -> Self::Output {
        unsafe {
            let mut buffer = <[T; N]>::default();

            let byte_length = core::mem::size_of_val(&buffer) as u32;

            let response_size = intrinsics::request_provider_response(
                buffer.as_mut_ptr() as _,
                byte_length,
                self.index as u32,
            );

            debug_assert_eq!(response_size, byte_length);

            buffer
        }
    }
}
