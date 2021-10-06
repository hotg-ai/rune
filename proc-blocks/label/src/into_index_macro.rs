use core::convert::TryInto;
use libm::floorf;

pub trait IntoIndex: Sized {
    fn try_into_index(self) -> Option<usize>;
    fn into_index(self) -> usize {
        self.try_into_index().expect("UNSUPPORTED: Can't be converted to usize. It only supports u8, u16, u32, u64, i32, i64 ( with positive numbers) f32 (with their fractional part zero E.g. 2.0, 4.0, etc)")
    }
}

macro_rules! float_into_index {
    ($($float:ty),*$(,)?) => {$(
        impl IntoIndex for $float {
            fn try_into_index(self) -> Option<usize> {
                // Integers are exactly representable at or below this value
                const MAX_VALUE: $float = (1u64 << <$float>::MANTISSA_DIGITS) as $float;

                if self >= 0.0 && self < MAX_VALUE && self-floorf(self) == 0_f32 {
                    (self as u32).try_into().ok()
                } else {
                    None
                }
            }
        }
    )*}
}

float_into_index!(f32);

macro_rules! integer_into_index {
    ($($int:ty),*$(,)?) => {$(
        impl IntoIndex for $int {
            fn try_into_index(self) -> Option<usize> {
                self.try_into().ok()
            }
        }
    )*}
}

integer_into_index!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
