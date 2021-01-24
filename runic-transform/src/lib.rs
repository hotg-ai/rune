#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use runic_types::*;
use alloc::vec::Vec;

pub trait Transformable<INPUT_BUFFER_TYPE, OUTPUT_BUFFER_TYPE> {
    fn to_buffer(input: &[INPUT_BUFFER_TYPE], input_size: usize) -> Vec<u8>;
    fn from_buffer(input: Vec<u8>, buffer_size: usize) -> Vec<INPUT_BUFFER_TYPE>;
}

pub struct Transform<InputType, OutputType> {
    _input: Option<InputType>,
    _output: Option<OutputType>
}

// Transformable<input_buffer_type, output_buffer_type> for &[Input Type]
impl Transformable<f32, i32> for Transform<f32, i32> {
    // Should return a Vec<i32> ??
    fn to_buffer(_input: &[f32], input_size: usize) -> Vec<u8> {
        // Transformed to &[f32] then to &[u8]
        
        return Vec::with_capacity(input_size * 4 as usize);
    }

    fn from_buffer(_input: Vec<u8>, buffer_size: usize) -> Vec<f32> {

        return Vec::from([0.0]);
    }
}

// Transformable<input_buffer_type, output_buffer_type> for &[Input Type]
impl Transformable<f32, f32> for Transform<f32, f32> {
    // Should return a Vec<i32> ??
    fn to_buffer(input: &[f32], input_size: usize) -> Vec<u8> {
        // Transformed to &[f32] then to &[u8]
        let layout = alloc::alloc::Layout::from_size_align(input_size * 4, 1).unwrap();
        let mut out: Vec<u8> = Vec::with_capacity(input_size*4);
        
        for input_idx in 0..input_size  {
           let input = input[input_idx];
           let input = input.to_be_bytes();
           out[input_idx + 0] = input[0];
           out[input_idx + 1] = input[1];
           out[input_idx + 2] = input[2];
           out[input_idx + 3] = input[3];
        }
        return out;
    }

    fn from_buffer(_input: Vec<u8>, buffer_size: usize) -> Vec<f32> {

        return Vec::with_capacity( (buffer_size / 4) as usize);
    }
}



#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn can_transform_same_types() {

       let raw: Vec<f32> = Vec::from([0.0]);
       let buffer: Vec<u8> = Transform::<f32, f32>::to_buffer(raw, raw.len());
       let transform: Vec<f32> = Transform::<f32, f32>::from_buffer(buffer);
       

       println!("{:?}", buffer);
    }
}
