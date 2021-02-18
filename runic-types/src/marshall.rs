use crate::proc_block::TYPE;
use alloc::vec::Vec;

pub trait Transformable<A> {
    fn transform(input: &Vec<u8>, data_type: TYPE) -> Vec<A>;
}
#[allow(dead_code)]
pub struct Transformed<T: Transformable<T>> {
    data_type: TYPE,
    input: Vec<u8>,
    output: Option<Vec<T>>,
}

impl<T: Transformable<T>> Transformed<T> {
    pub fn to(&self, to_data_type: TYPE) -> Vec<T> {
        let ret: Vec<T> = T::transform(&self.input, to_data_type);
        return ret;
    }
}

pub fn marshall<T: Transformable<T>>(
    data_type: TYPE,
    input: Vec<u8>,
) -> Transformed<T> {
    return Transformed::<T> {
        data_type,
        input,
        output: None,
    };
}
