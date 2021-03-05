use crate::AsParamType;

/// A generic buffer that can be used for transferring data between the Rune
/// and the runtime.
pub trait Buffer: Sized {
    type Item: AsParamType;
    const OVERALL_LENGTH: usize;

    fn zeroed() -> Self;
    fn as_ptr(&self) -> *const Self::Item;
    fn as_mut_ptr(&mut self) -> *mut Self::Item;
    fn as_slice(&self) -> &[Self::Item];
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

/// Gets the type name for a multidimensional array (e.g. `[[[f32; A]; B], C]`).
macro_rules! array_type {
    ($ty:ty, $first_dim:ident $(, $dims:ident)* $(,)?) => {
        [array_type!($ty, $($dims),*) ; $first_dim]
    };
    ($ty:ty $(,)?) => {
        $ty
    };
}

/// Generates an array with the appropriate dimensions filled with zeroes
/// (e.g. `[[[0.0; A]; B], C]`).
macro_rules! array_zeroed {
    ($ty:ty, $first_dim:ident $(, $dims:ident)* $(,)?) => {
        [array_zeroed!($ty, $($dims),*) ; $first_dim]
    };
    ($ty:ty $(,)?) => {
        <$ty>::default()
    };
}

macro_rules! impl_buffer {
    ($underlying_type:ty, $first:ident $(, $other_dims:ident)* $(,)?) => {
        impl<const $first:usize, $(const $other_dims : usize),*> Buffer for
            array_type!($underlying_type, $first $(, $other_dims)*)
        {
            type Item = $underlying_type;
            const OVERALL_LENGTH: usize = $first $(* $other_dims)*;

            fn zeroed() -> Self {
                array_zeroed!($underlying_type, $first, $($other_dims),*)
            }

            fn as_ptr(&self) -> *const Self::Item {
                self[..].as_ptr().cast()
            }

            fn as_mut_ptr(&mut self) -> *mut Self::Item {
                self[..].as_mut_ptr().cast()
            }

            fn as_slice(&self) -> &[Self::Item] {
                unsafe {
                    core::slice::from_raw_parts(self.as_ptr(), Self::OVERALL_LENGTH)
                }
            }

            fn as_mut_slice(&mut self) -> &mut [Self::Item] {
                unsafe {
                    core::slice::from_raw_parts_mut(
                        self.as_mut_ptr(),
                        Self::OVERALL_LENGTH,
                    )
                }
            }
        }

        impl_buffer!($underlying_type, $($other_dims),*);
    };
    ($underlying_type:ty $(,)?) => {};
}

impl_buffer!(f32, A, B, C, D);
impl_buffer!(i32, A, B, C, D);
