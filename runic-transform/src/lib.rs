#![no_std]

#[macro_use]
extern crate alloc;

// use alloc::boxed::Box;
// use runic_types::*;
use alloc::vec::Vec;

fn chunk_to_typed<T: Clone, F>(
    input: &Vec<u8>,
    chunk_size: usize,
    transform: F,
) -> Vec<T>
where
    F: for<'a> Fn(&[u8]) -> T,
{
    return input.chunks(chunk_size).fold(vec![], |mut out_vec, chunk| {
        let atom: T = transform(chunk);
        out_vec.push(atom as T);
        return out_vec;
    });
}

pub trait Transformable<InputBufferType, OutputBufferType> {
    fn to_buffer(input: &Vec<InputBufferType>)
        -> Result<Vec<u8>, &'static str>;
    fn from_buffer(
        input: &Vec<u8>,
    ) -> Result<Vec<OutputBufferType>, &'static str>;
}

pub struct Transform<InputType, OutputType> {
    _input: Option<InputType>,
    _output: Option<OutputType>,
}

// Transformable<InputBufferType, OutputBufferType> for &[Input Type]
impl Transformable<f32, i32> for Transform<f32, i32> {
    // Should return a Vec<i32> ??
    fn to_buffer(input: &Vec<f32>) -> Result<Vec<u8>, &'static str> {
        // Transformed to &[f32] then to &[u8]
        // Transformed to &[f32] then to &[u8]
        let input_size = input.len();
        if input_size == 0 {
            return Ok(Vec::from([]));
        }
        let out_size = input_size * 4;
        let mut out: Vec<u8> = Vec::with_capacity(out_size);

        for input_idx in 0..input_size {
            let input = input[input_idx];
            let input: i32 = input as i32;
            let input = input.to_be_bytes();
            out.push(input[0]);
            out.push(input[1]);
            out.push(input[2]);
            out.push(input[3]);
        }
        return Ok(out);
    }

    fn from_buffer(_input: &Vec<u8>) -> Result<Vec<i32>, &'static str> {
        let out = chunk_to_typed(&_input, 4, |chunk| {
            let f =
                i32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            return f as i32;
        });
        return Ok(out);
    }
}

// Transformable<InputBufferType, OutputBufferType> for &[Input Type]
impl Transformable<i32, i32> for Transform<i32, i32> {
    // Should return a Vec<i32> ??
    fn to_buffer(input: &Vec<i32>) -> Result<Vec<u8>, &'static str> {
        // Transformed to &[f32] then to &[u8]
        // Transformed to &[f32] then to &[u8]
        let input_size = input.len();
        if input_size == 0 {
            return Ok(Vec::from([]));
        }
        let out_size = input_size * 4;
        let mut out: Vec<u8> = Vec::with_capacity(out_size);

        for input_idx in 0..input_size {
            let input = input[input_idx];
            let input = input.to_be_bytes();
            out.push(input[0]);
            out.push(input[1]);
            out.push(input[2]);
            out.push(input[3]);
        }
        return Ok(out);
    }

    fn from_buffer(_input: &Vec<u8>) -> Result<Vec<i32>, &'static str> {
        let out = chunk_to_typed(&_input, 4, |chunk| {
            let f =
                i32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            return f as i32;
        });
        return Ok(out);
    }
}

// Transformable<InputBufferType, OutputBufferType> for &[Input Type]
impl Transformable<i32, i16> for Transform<i32, i16> {
    // Should return a Vec<i32> ??
    fn to_buffer(input: &Vec<i32>) -> Result<Vec<u8>, &'static str> {
        // Transformed to &[f32] then to &[u8]
        // Transformed to &[f32] then to &[u8]
        let input_size = input.len();
        if input_size == 0 {
            return Ok(Vec::from([]));
        }
        let out_size = input_size * 2;
        let mut out: Vec<u8> = Vec::with_capacity(out_size);

        for input_idx in 0..input_size {
            let input = input[input_idx];
            let input: i16 = (input as i16).into();
            let input = input.to_be_bytes();
            out.push(input[0]);
            out.push(input[1]);
        }
        return Ok(out);
    }

    fn from_buffer(_input: &Vec<u8>) -> Result<Vec<i16>, &'static str> {
        let out = chunk_to_typed(&_input, 2, |chunk| {
            let f = i16::from_be_bytes([chunk[0], chunk[1]]);
            return f as i16;
        });
        return Ok(out);
    }
}

// Transformable<InputBufferType, OutputBufferType> for &[Input Type]
impl Transformable<f32, f32> for Transform<f32, f32> {
    // Should return a Vec<i32> ??
    fn to_buffer(input: &Vec<f32>) -> Result<Vec<u8>, &'static str> {
        // Transformed to &[f32] then to &[u8]
        let input_size = input.len();
        if input_size == 0 {
            return Ok(Vec::from([]));
        }
        let out_size = input_size * 4;
        let mut out: Vec<u8> = Vec::with_capacity(out_size);

        for input_idx in 0..input_size {
            let input = input[input_idx];

            let input = input.to_be_bytes();

            out.push(input[0]);
            out.push(input[1]);
            out.push(input[2]);
            out.push(input[3]);
        }
        return Ok(out);
    }

    fn from_buffer(_input: &Vec<u8>) -> Result<Vec<f32>, &'static str> {
        if _input.len() == 0 {
            return Ok(Vec::with_capacity(0));
        }
        assert_eq!(_input.len() % 4, 0);
        if _input.len() % 4 != 0 {
            return Err("Input length needs to be divisible by 4 ");
        }
        let out = chunk_to_typed(&_input, 4, |chunk| {
            let f =
                f32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            return f;
        });
        return Ok(out);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::alloc::borrow::ToOwned;
    use crate::*;
    use rand::{thread_rng, Rng};
    #[test]
    fn can_transform_same_types() {
        let mut rng = thread_rng();
        let raw: Vec<f32> =
            Vec::from([rng.gen(), rng.gen(), rng.gen(), rng.gen()]);
        let buffer: Vec<u8> = Transform::<f32, f32>::to_buffer(&raw).unwrap();

        let transform: Vec<f32> =
            Transform::<f32, f32>::from_buffer(&buffer).unwrap();

        assert_eq!(raw, transform);
    }

    #[test]
    fn can_transform_f32_i32() {
        let raw: Vec<f32> = Vec::from([1.2, 12.2, 1231.2, -633.12, 78432.2]);
        // Outputs Vec<u8> which has encode Vec<i32>
        let buffer: Vec<u8> = Transform::<f32, i32>::to_buffer(&raw).unwrap();

        let transform: Vec<i32> =
            Transform::<f32, i32>::from_buffer(&buffer).unwrap();

        assert_eq!(Vec::from([1, 12, 1231, -633, 78432]), transform);
    }
}
