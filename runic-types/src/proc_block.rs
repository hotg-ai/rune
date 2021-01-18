
use alloc::string::String;
use alloc::vec::Vec;

fn chunk_to_typed<T: Clone, F>(input: &Vec<u8>, chunk_size: usize, transform: F) -> Vec<T>
where
    F: for<'a> Fn(&[u8]) -> T,
{
    return input.chunks(chunk_size).fold(vec![], |mut out_vec, chunk| {
        let atom: T = transform(chunk);
        out_vec.push(atom as T);
        return out_vec;
    });
}

fn vi32_to_vi16(input: &Vec<u8>) -> Vec<i16> {
    return chunk_to_typed(input, 2, |chunk| {
        return i16::from_be_bytes([chunk[0], chunk[1]]);
    });
}

fn vf32_to_vi16(input: &Vec<u8>) -> Vec<i16> {
    return chunk_to_typed(input, 4, |chunk| {
        let f32_atom: f32 = f32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        return f32_atom as i16;
    });
}

fn vf64_to_vi16(input: &Vec<u8>) -> Vec<i16> {
    return chunk_to_typed(input, 8, |chunk| {
        let f64_atom: f64 = f64::from_be_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
        ]);
        return f64_atom as i16;
    });
}

pub enum TYPE {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    UTF8,
}
pub struct TransformableType {}

impl crate::marshall::Transformable<f32> for TransformableType {
    // add code here
    fn transform(input: &Vec<u8>, data_type: TYPE) -> Vec<f32> {
        return match data_type {
            TYPE::F32 => {
                return chunk_to_typed(input, 4, |chunk| {
                    f32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                })
            }
            TYPE::F64 => {
                return chunk_to_typed(input, 8, |chunk| {
                    let f64_atom = f64::from_be_bytes([
                        chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6],
                        chunk[7],
                    ]);
                    return f64_atom as f32;
                });
            }
            TYPE::I32 => {
                return chunk_to_typed(input, 4, |chunk| {
                    let i32_atom = i32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    return i32_atom as f32;
                });
            }
            _ => vec![],
        };
    }
}

impl crate::marshall::Transformable<i16> for TransformableType {
    // add code here
    fn transform(input: &Vec<u8>, data_type: TYPE) -> Vec<i16> {
        return match data_type {
            TYPE::F32 => return vf32_to_vi16(input),
            TYPE::F64 => return vf64_to_vi16(input),
            TYPE::I32 => return vi32_to_vi16(input),
            _ => vec![],
        };
    }
}

pub trait ProcBlock {
    fn process(
        &self,
        input_t: TYPE,
        input: Vec<u8>,
        params: alloc::collections::BTreeMap<String, String>,
        output_t: TYPE,
    ) -> Vec<u8>;
}
