use core::convert::TryInto;
use libm::floorf;

pub trait IntoIndex: Sized {
    fn try_into_index(self) -> usize;
}

macro_rules! float_into_index {
    ($($float:ty),*$(,)?) => {$(
        impl IntoIndex for $float {
            fn try_into_index(self) -> usize {
                // Integers are exactly representable at or below this value
                const MAX_VALUE: $float = (1u64 << <$float>::MANTISSA_DIGITS) as $float;

                assert!(!self.is_nan(),"The index can't be NAN");
                assert!(!self.is_infinite(), "The index can't be infinite");
                assert!(self >= 0.0, "The index must be a positive number");
                assert!(self <= MAX_VALUE, "The index is larger than the largest number that can safely represent an integer");
                assert_eq!(self - floorf(self), 0.0, "The index wasn't an integer");

                (self as u32).try_into().expect("UNSUPPORTED: Can't be converted to usize. It only supports u8, u16, u32, u64, i32, i64 ( with positive numbers) f32 (with their fractional part zero E.g. 2.0, 4.0, etc)")
            }
        }
    )*}
}

float_into_index!(f32);

macro_rules! integer_into_index {
    ($($int:ty),*$(,)?) => {$(
        impl IntoIndex for $int {
            fn try_into_index(self) -> usize {
                self.try_into().ok().expect("UNSUPPORTED: Can't be converted to usize. It only supports u8, u16, u32, u64, i32, i64 ( with positive numbers) f32 (with their fractional part zero E.g. 2.0, 4.0, etc)")
            }
        }
    )*}
}

integer_into_index!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_floats() { assert_eq!(1, 1.0f32.try_into_index()) }

    #[test]
    fn test_u32_inetger() { assert_eq!(1, 1u32.try_into_index()) }

    #[test]
    #[should_panic]
    fn test_negative_integer() { (-1).try_into_index(); }

    #[test]
    #[should_panic]
    fn test_infinite() { (1.0 / 0.0).try_into_index(); }

    #[test]
    #[should_panic = "The index must be a positive number"]
    fn test_negative_float() { (-3.0).try_into_index(); }
    #[test]
    #[should_panic]
    fn test_float_with_fraction() { (4.3).try_into_index(); }

    #[test]
    #[should_panic]
    fn test_float_out_of_bound() {
        (16_777_216_456_673_784.0).try_into_index();
    }
}
